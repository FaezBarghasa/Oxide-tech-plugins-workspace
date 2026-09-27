from skidl import *
from oxide_tech_client import OxideClient
import logging

logger = logging.getLogger("OxideSchematicGenerator")

class Schematic:
    def __init__(self, netlist: str):
        self.netlist = netlist

    @classmethod
    def from_netlist(cls, netlist: str):
        return cls(netlist)

class ERCResults:
    def __init__(self, errors=None):
        self.errors = errors or []

class OxideSchematicGenerator:
    def __init__(self, project_id: str):
        self.client = OxideClient(base_url="https://llm.oxide-tech.com")
        self.project_id = project_id
        self.ctx = self.client.get_project_context(project_id)
    
    def generate_from_ai(self, spec: str) -> Schematic:
        """Ask AI to generate schematic from natural language spec"""
        response = self.client.inference(
            prompt=f"Generate SKiDL code for: {spec}",
            model="qwen-32b",
            context=self.ctx,
        )
        
        # Execute AI-generated SKiDL code in sandbox
        schematic = self.execute_skidl_sandbox(response.code)
        
        # Run ERC
        erc_results = self.run_erc(schematic)
        if erc_results.errors:
            # Ask AI to fix errors
            fixed = self.client.inference(
                prompt=f"Fix these ERC errors: {erc_results.errors}",
                model="qwen-32b",
                context={"code": response.code, "errors": erc_results.errors},
            )
            schematic = self.execute_skidl_sandbox(fixed.code)
        
        # Update SurrealDB graph
        self.client.sync_netlist_to_graph(schematic.netlist)
        
        return schematic
    
    def run_erc(self, schematic: Schematic) -> ERCResults:
        """Run electrical rules check using SKiDL"""
        errors = []
        try:
            import skidl
            # Run simple net check on default_circuit nets
            for net in skidl.default_circuit.nets:
                if len(net.pins) < 2:
                    errors.append(f"ERC Warning: Net {net.name} has less than 2 pins connected ({len(net.pins)})")
        except Exception as e:
            errors.append(f"ERC Exception: {str(e)}")
        return ERCResults(errors)

    def execute_skidl_sandbox(self, code: str) -> Schematic:
        """Execute SKiDL in isolated Docker container"""
        import docker
        client = docker.from_env()
        container = client.containers.run(
            image="oxide/kicad-sandbox:latest",
            command=["python3", "-c", code],
            mem_limit="2g",
            network_mode="none",
            detach=False,
            remove=True,
        )
        return Schematic.from_netlist(container.decode())
