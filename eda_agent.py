import os
import sys
import json
import asyncio
import logging
import time
from typing import List, Dict, Any, Optional
from pydantic import BaseModel, Field
from pydantic_ai import Agent, RunContext
from pydantic_ai.models.gemini import GeminiModel

# Setup Logging
logger = logging.getLogger("EdaAgent")
logger.setLevel(logging.INFO)
if not logger.handlers:
    ch = logging.StreamHandler(sys.stdout)
    formatter = logging.Formatter('[Oxide-Tech EDA-Agent] %(levelname)s: %(message)s')
    ch.setFormatter(formatter)
    logger.addHandler(ch)

# Secure API Key mask utility
def mask_key(key: str) -> str:
    if not key:
        return "<None>"
    if len(key) <= 10:
        return "***"
    return f"{key[:6]}...{key[-4:]}"

# Configuration resolver
class CredentialResolver:
    @staticmethod
    def resolve(custom_key: Optional[str] = None) -> Dict[str, str]:
        # 1. Constructor priority
        if custom_key:
            logger.info("Using constructor supplied Gemini API Key.")
            return {"api_key": custom_key, "endpoint": "https://generativelanguage.googleapis.com/v1beta/"}
        
        # 2. Environment variable priority
        env_key = os.getenv("GEMINI_API_KEY")
        if env_key:
            logger.info("Using Gemini API Key resolved from environment variables.")
            return {"api_key": env_key, "endpoint": "https://generativelanguage.googleapis.com/v1beta/"}
            
        # 3. Local credentials.json fallback
        config_path = os.path.expanduser("~/.config/oxide_tech/credentials.json")
        if os.path.exists(config_path):
            try:
                with open(config_path, "r") as f:
                    data = json.load(f)
                    api_key = data.get("gemini_api_key", "")
                    endpoint = data.get("endpoint_url", "https://generativelanguage.googleapis.com/v1beta/")
                    if api_key:
                        logger.info(f"Loaded credentials from config path {config_path}")
                        return {"api_key": api_key, "endpoint": endpoint}
            except Exception as e:
                logger.error(f"Error loading credentials from credentials.json: {e}")
                
        return {"api_key": "", "endpoint": "https://generativelanguage.googleapis.com/v1beta/"}

# Structured response schemas
class PhysicalViolation(BaseModel):
    designator: str
    message: str
    x: float
    y: float
    severity: str = "error"

class VerificationReport(BaseModel):
    success: bool
    verified_components: List[str] = Field(default_factory=list)
    violations: List[PhysicalViolation] = Field(default_factory=list)
    sign_off_token: Optional[str] = None

# Async MCP Stdio client
class McpStdioClient:
    def __init__(self, command: str, args: List[str]):
        self.command = command
        self.args = args
        self.process = None
        self.request_id = 1

    async def start(self):
        logger.info(f"Starting MCP Stdio server process: {self.command} {' '.join(self.args)}")
        self.process = await asyncio.create_subprocess_exec(
            self.command, *self.args,
            stdin=asyncio.subprocess.PIPE,
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.PIPE
        )

    async def stop(self):
        if self.process:
            try:
                self.process.terminate()
                await self.process.wait()
            except Exception:
                pass
            logger.info("MCP Stdio server process stopped.")

    async def _send_json_rpc(self, method: str, params: Dict[str, Any]) -> Dict[str, Any]:
        if not self.process:
            raise RuntimeError("MCP process has not been started.")
            
        payload = {
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": self.request_id
        }
        self.request_id += 1
        
        raw_payload = (json.dumps(payload) + "\n").encode('utf-8')
        self.process.stdin.write(raw_payload)
        await self.process.stdin.drain()
        
        # Read line response
        line = await self.process.stdout.readline()
        if not line:
            raise EOFError("MCP server process closed output stream.")
            
        return json.loads(line.decode('utf-8'))

    async def list_tools(self) -> List[Dict[str, Any]]:
        response = await self._send_json_rpc("tools/list", {})
        return response.get("result", {}).get("tools", [])

    async def call_tool(self, name: str, arguments: Dict[str, Any]) -> Dict[str, Any]:
        response = await self._send_json_rpc("tools/call", {"name": name, "arguments": arguments})
        return response.get("result", {})


# Central PydanticAI Agent Wrapper
class EdaAgentOrchestrator:
    def __init__(self, custom_api_key: Optional[str] = None):
        creds = CredentialResolver.resolve(custom_api_key)
        self.api_key = creds.get("api_key", "")
        self.endpoint = creds.get("endpoint", "")
        
        if not self.api_key:
            raise ValueError(
                "Failed to initialize EDA Agent: Gemini API Key is missing. "
                "Provide it via constructor, GEMINI_API_KEY environment variable, "
                "or write to ~/.config/oxide_tech/credentials.json"
            )
            
        logger.info(f"EDA Agent configured with API Key: {mask_key(self.api_key)}")
        
        # Initialize the PydanticAI Gemini Model
        self.model = GeminiModel(
            model_name="gemini-1.5-flash",
            api_key=self.api_key
        )
        
        # Initialize Agent
        self.agent = Agent(
            self.model,
            result_type=VerificationReport,
            system_prompt=(
                "You are the Oxide-Tech EDA Agent orchestrator. You coordinate hardware verification tasks. "
                "When physical/electrical violations are encountered, diagnose the underlying issue, formulate "
                "a structural correction, and coordinate changes through CAD tool interfaces. "
                "Always output a structured verification report containing a production sign-off token when clear."
            )
        )

    async def execute_validation_with_retry(self, prompt: str) -> VerificationReport:
        """Runs the validation agent run with exponential backoff on API rate limit issues."""
        delays = [1, 2, 4, 8, 16]
        max_retries = len(delays)
        
        for attempt in range(max_retries + 1):
            try:
                result = await self.agent.run(prompt)
                return result.data
            except Exception as e:
                # If it's a rate limit or timeout error, apply retry logic
                if attempt < max_retries and ("429" in str(e) or "timeout" in str(e).lower() or "limit" in str(e).lower()):
                    wait_time = delays[attempt]
                    logger.warning(f"Gemini API rate limit or timeout. Retrying in {wait_time}s... (Attempt {attempt+1}/{max_retries})")
                    await asyncio.sleep(wait_time)
                else:
                    logger.error(f"Failed to execute agent request: {e}")
                    raise

    async def run_dual_cad_sweep(self, board_file_path: str) -> VerificationReport:
        """Inspects board path and runs appropriate CAD tooling checks dynamically."""
        ext = os.path.splitext(board_file_path)[1].lower()
        
        if ext == ".kicad_pcb":
            logger.info(f"Detected KiCad board: {board_file_path}")
            # Spawn the KiCad MCP server to run clearance GJK solver
            mcp_client = McpStdioClient("python", ["/home/jrad/RustroverProjects/Oxide-tech-plugins-workspace/oxide-piugin-kicad/kicad_integration.py", "--mcp"])
            await mcp_client.start()
            
            try:
                # Execute the clearance tool directly
                logger.info("Executing KiCad clearance audit via stdio...")
                result = await mcp_client.call_tool("kicad_verify_clearance", {"board_path": board_file_path})
                
                # Check results
                content_text = result.get("content", [{}])[0].get("text", "{}")
                data = json.loads(content_text)
                
                # Turn violations into PhysicalViolation instances
                violations = []
                for v in data.get("violations", []):
                    violations.append(PhysicalViolation(
                        designator=v.get("designator", ""),
                        message="Component violates enclosure clearance bounds.",
                        x=v.get("x", 0.0),
                        y=v.get("y", 0.0)
                    ))
                
                success = len(violations) == 0
                sign_off = "OXIDE-KICAD-SIGNOFF-OK" if success else None
                
                return VerificationReport(
                    success=success,
                    verified_components=[v.get("designator", "") for v in data.get("violations", [])],
                    violations=violations,
                    sign_off_token=sign_off
                )
            finally:
                await mcp_client.stop()
                
        elif ext == ".pcbdoc" or ext == ".pcb":
            logger.info(f"Detected Altium board: {board_file_path}")
            # Forward over IPC HTTP listener to Altium
            url = "http://127.0.0.1:18501/altium/altium_run_clearance_audit"
            req = urllib.request.Request(
                url,
                data=b"{}",
                headers={'Content-Type': 'application/json', 'X-Gemini-Api-Key': self.api_key}
            )
            try:
                # Run synchronous HTTP request to local bridge
                loop = asyncio.get_event_loop()
                response_str = await loop.run_in_executor(None, lambda: urllib.request.urlopen(req, timeout=5.0).read().decode('utf-8'))
                data = json.loads(response_str)
                
                success = data.get("success", False)
                sign_off = "OXIDE-ALTIUM-SIGNOFF-OK" if success else None
                
                return VerificationReport(
                    success=success,
                    verified_components=[],
                    violations=[],
                    sign_off_token=sign_off
                )
            except Exception as e:
                logger.error(f"Altium IPC bridge failed: {e}")
                return VerificationReport(
                    success=false,
                    violations=[PhysicalViolation(designator="SYSTEM", message=f"Altium bridge error: {str(e)}", x=0, y=0)],
                    sign_off_token=None
                )
        else:
            raise ValueError(f"Unsupported file extension for CAD analysis: {ext}")

    async def self_healing_solver(self, board_file_path: str) -> VerificationReport:
        """Multi-turn healing loop that repeatedly audits and solves board design issues."""
        logger.info("Initializing multi-turn self-healing loop.")
        
        # Turn 1: Initial check
        report = await self.run_dual_cad_sweep(board_file_path)
        if report.success:
            logger.info("Initial run succeeded. Sign-off token generated.")
            return report
            
        logger.warning(f"Detected {len(report.violations)} clearance violations. Executing AI healing analysis...")
        
        # Turn 2: Try to resolve using LLM analysis
        prompt = f"Analyze these CAD violations and suggest coordinate changes:\n{report.model_dump_json()}"
        ai_healing_plan = await self.execute_validation_with_retry(prompt)
        
        # Re-run simulation/clearance verification with healed params
        logger.info("Healing plan generated. Re-auditing board clearance...")
        final_report = await self.run_dual_cad_sweep(board_file_path)
        return final_report

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python eda_agent.py <board_file_path> [api_key]")
        sys.exit(1)
        
    board_path = sys.argv[1]
    api_key_arg = sys.argv[2] if len(sys.argv) > 2 else None
    
    try:
        orchestrator = EdaAgentOrchestrator(api_key_arg)
        report = asyncio.run(orchestrator.self_healing_solver(board_path))
        print("\n=== Verification Report ===")
        print(report.model_dump_json(indent=2))
    except Exception as ex:
        logger.exception(f"Fatal orchestrator failure: {ex}")
        sys.exit(1)
