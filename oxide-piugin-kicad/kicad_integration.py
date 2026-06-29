import sys
import os
import json
import logging
import stat
import tkinter as tk
from tkinter import ttk, messagebox
import urllib.request
import urllib.error
import subprocess

# JIT Import resolution path
uniffi_path = "/usr/lib/oxide_tech/bindings/python"
if uniffi_path not in sys.path:
    sys.path.append(uniffi_path)

# Configure logging
logger = logging.getLogger("OxideTechKiCad")
logger.setLevel(logging.INFO)
if not logger.handlers:
    ch = logging.StreamHandler(sys.stdout)
    formatter = logging.Formatter('[Oxide-Tech KiCad] %(levelname)s: %(message)s')
    ch.setFormatter(formatter)
    logger.addHandler(ch)

# Credentials path
CREDENTIALS_DIR = os.path.expanduser("~/.config/oxide_tech")
CREDENTIALS_PATH = os.path.join(CREDENTIALS_DIR, "credentials.json")

def load_credentials():
    """Loads stored credentials, returns dictionary of key and endpoint."""
    if not os.path.exists(CREDENTIALS_PATH):
        return {"gemini_api_key": "", "endpoint_url": "https://generativelanguage.googleapis.com/v1beta/"}
    
    try:
        # Check permissions - must be user-only read/write
        file_stat = os.stat(CREDENTIALS_PATH)
        if file_stat.st_mode & 0o077:
            logger.warning("Credentials file has insecure permissions. Restricting to user-only access.")
            os.chmod(CREDENTIALS_PATH, stat.S_IRUSR | stat.S_IWUSR)
            
        with open(CREDENTIALS_PATH, "r") as f:
            data = json.load(f)
            return {
                "gemini_api_key": data.get("gemini_api_key", ""),
                "endpoint_url": data.get("endpoint_url", "https://generativelanguage.googleapis.com/v1beta/")
            }
    except Exception as e:
        logger.error(f"Failed to read credentials file: {e}")
        return {"gemini_api_key": "", "endpoint_url": "https://generativelanguage.googleapis.com/v1beta/"}

def save_credentials(api_key, endpoint_url):
    """Saves credentials securely with restricted permissions."""
    try:
        if not os.path.exists(CREDENTIALS_DIR):
            os.makedirs(CREDENTIALS_DIR, mode=0o700, exist_ok=True)
            
        payload = {
            "gemini_api_key": api_key,
            "endpoint_url": endpoint_url
        }
        
        with open(CREDENTIALS_PATH, "w") as f:
            json.dump(payload, f, indent=4)
            
        # Secure the file (chmod 600 - owner read/write only)
        os.chmod(CREDENTIALS_PATH, stat.S_IRUSR | stat.S_IWUSR)
        logger.info(f"Credentials successfully saved to {CREDENTIALS_PATH}")
        return True
    except Exception as e:
        logger.error(f"Failed to save credentials file: {e}")
        return False

def validate_api_key(api_key, endpoint_url):
    """Hits Google AI Studio v1beta models endpoint to validate API key."""
    if not api_key:
        return False, "API Key is empty."
    
    url = f"{endpoint_url.rstrip('/')}/models?key={api_key}"
    try:
        req = urllib.request.Request(url, headers={'User-Agent': 'Oxide-Tech-KiCad-Validator'})
        with urllib.request.urlopen(req, timeout=5.0) as response:
            if response.getcode() == 200:
                return True, "API Key validated successfully."
    except urllib.error.HTTPError as e:
        try:
            err_data = json.loads(e.read().decode('utf-8'))
            err_msg = err_data.get("error", {}).get("message", "HTTP Error")
            return False, f"Validation failed: {err_msg}"
        except Exception:
            return False, f"Validation failed with HTTP status code {e.code}"
    except Exception as e:
        return False, f"Validation request failed: {str(e)}"
    
    return False, "Unknown validation response."


class ByokSettingsDialog:
    """Tkinter interface for securely managing Gemini API Key / BYOK settings."""
    def __init__(self, parent=None):
        self.root = parent if parent else tk.Tk()
        if not parent:
            self.root.title("Oxide-Tech BYOK Settings")
            self.root.geometry("450x260")
            self.root.resizable(False, False)
            
        self.credentials = load_credentials()
        self.api_key_var = tk.StringVar(value=self.credentials.get("gemini_api_key", ""))
        self.endpoint_var = tk.StringVar(value=self.credentials.get("endpoint_url", ""))
        
        self.setup_ui()
        
    def setup_ui(self):
        main_frame = ttk.Frame(self.root, padding="15 15 15 15")
        main_frame.pack(fill=tk.BOTH, expand=True)
        
        title_label = ttk.Label(main_frame, text="Oxide-Tech BYOK Security Settings", font=("Helvetica", 12, "bold"))
        title_label.grid(row=0, column=0, columnspan=2, pady=(0, 15), sticky="w")
        
        ttk.Label(main_frame, text="Gemini API Key:").grid(row=1, column=0, sticky="w", pady=5)
        self.api_key_entry = ttk.Entry(main_frame, textvariable=self.api_key_var, show="*", width=35)
        self.api_key_entry.grid(row=1, column=1, sticky="w", pady=5, padx=(10, 0))
        
        ttk.Label(main_frame, text="Model Endpoint:").grid(row=2, column=0, sticky="w", pady=5)
        self.endpoint_entry = ttk.Entry(main_frame, textvariable=self.endpoint_var, width=35)
        self.endpoint_entry.grid(row=2, column=1, sticky="w", pady=5, padx=(10, 0))
        
        btn_frame = ttk.Frame(main_frame)
        btn_frame.grid(row=3, column=0, columnspan=2, pady=(20, 0), sticky="e")
        
        self.validate_btn = ttk.Button(btn_frame, text="Validate Key", command=self.on_validate)
        self.validate_btn.pack(side=tk.LEFT, padx=5)
        
        self.save_btn = ttk.Button(btn_frame, text="Save Settings", command=self.on_save)
        self.save_btn.pack(side=tk.LEFT, padx=5)
        
        self.close_btn = ttk.Button(btn_frame, text="Close", command=self.root.destroy)
        self.close_btn.pack(side=tk.LEFT, padx=5)
        
    def on_validate(self):
        api_key = self.api_key_var.get().strip()
        endpoint = self.endpoint_var.get().strip()
        
        self.validate_btn.config(state=tk.DISABLED)
        self.root.update_idletasks()
        
        success, message = validate_api_key(api_key, endpoint)
        self.validate_btn.config(state=tk.NORMAL)
        
        if success:
            messagebox.showinfo("Success", message, parent=self.root)
        else:
            messagebox.showerror("Validation Failed", message, parent=self.root)
            
    def on_save(self):
        api_key = self.api_key_var.get().strip()
        endpoint = self.endpoint_var.get().strip()
        
        if not api_key:
            messagebox.showwarning("Warning", "API key cannot be empty.", parent=self.root)
            return
            
        if save_credentials(api_key, endpoint):
            messagebox.showinfo("Saved", "Settings successfully saved with secure permissions.", parent=self.root)
            self.root.destroy()
        else:
            messagebox.showerror("Error", "Could not save credentials to filesystem.", parent=self.root)

    def run(self):
        self.root.mainloop()


def perform_board_clearance_sweep(board):
    """Performs native bounding box calculation and footprint clearance logic."""
    import oxide_core
    
    if not board:
        return False, "No active board found in KiCad context.", []
        
    try:
        bounding_box = board.GetBoardEdgesBoundingBox()
        if bounding_box.GetWidth() == 0 or bounding_box.GetHeight() == 0:
            return False, "Board outline dimensions are zero. Ensure outline is drawn.", []
            
        width_mm = bounding_box.GetWidth() / 1000000.0
        length_mm = bounding_box.GetHeight() / 1000000.0
        
        physical_dim = oxide_core.PhysicalDimension(
            width=width_mm,
            length=length_mm,
            wall_thickness=2.0,
            clearance=1.5
        )
        
        controller = oxide_core.SystemController(physical_dim)
        logger.info(f"Initialized oxide_core SystemController. Bounding Box: {width_mm:.2f}mm x {length_mm:.2f}mm")
    except Exception as e:
        return False, f"Failed to initialize oxide_core bindings: {str(e)}", []

    collisions = []
    try:
        origin = board.GetDesignSettings().GetAuxOrigin()
        footprints = board.GetFootprints()
        
        for footprint in footprints:
            designator = footprint.GetReference()
            pos = footprint.GetPosition()
            
            x_mm = (pos.x - origin.x) / 1000000.0
            y_mm = (pos.y - origin.y) / 1000000.0
            
            height_mm = 0.0
            if footprint.HasProperty("Height"):
                height_str = footprint.GetProperty("Height")
                try:
                    height_mm = float(height_str.replace('mm', '').strip())
                except ValueError:
                    pass
            elif footprint.HasProperty("Height_Max"):
                height_str = footprint.GetProperty("Height_Max")
                try:
                    height_mm = float(height_str.replace('mm', '').strip())
                except ValueError:
                    pass
            
            if height_mm > 0.0:
                collision_detected = controller.run_gjk_clearance_solver(
                    designator=designator,
                    x=x_mm,
                    y=y_mm,
                    height=height_mm
                )
                if collision_detected:
                    collisions.append({
                        "designator": designator,
                        "x": x_mm,
                        "y": y_mm,
                        "height": height_mm
                    })
    except Exception as e:
        return False, f"Error iterating components: {str(e)}", []
        
    return True, "Clearance sweep complete.", collisions


def spawn_eda_agent(board_path):
    """Spawns the central eda-agent orchestrator as a subprocess, injecting BYOK credentials."""
    creds = load_credentials()
    api_key = creds.get("gemini_api_key")
    
    env = os.environ.copy()
    if api_key:
        env["GEMINI_API_KEY"] = api_key
        
    agent_script = "/home/jrad/RustroverProjects/Oxide-tech-plugins-workspace/eda_agent.py"
    cmd = [sys.executable, agent_script, board_path]
    
    logger.info(f"Spawning eda-agent subprocess for board: {board_path}")
    try:
        result = subprocess.run(cmd, env=env, capture_output=True, text=True, check=True)
        return True, result.stdout
    except subprocess.CalledProcessError as e:
        logger.error(f"eda-agent subprocess failed. Exit code: {e.returncode}\nError: {e.stderr}")
        return False, e.stderr
    except Exception as e:
        logger.error(f"Failed to spawn eda-agent subprocess: {str(e)}")
        return False, str(e)


# KiCad Action Plugin declaration (loaded by KiCad GUI)
try:
    import pcbnew
    
    class OxideCoreVerifier(pcbnew.ActionPlugin):
        def defaults(self):
            self.name = "Oxide-Tech Core Verifier"
            self.category = "Verification"
            self.description = "Dioxus v7 Native UI + SurrealDB v3 GJK clearance checks."
            self.show_toolbar_button = True
            self.icon_file_name = os.path.join(os.path.dirname(__file__), 'icon.png')
            
        def Run(self):
            logger.info("Starting KiCad Action Plugin execution...")
            
            creds = load_credentials()
            if not creds.get("gemini_api_key"):
                logger.warning("No Gemini API key found. Opening configuration dialog.")
                dialog = ByokSettingsDialog()
                dialog.run()
                creds = load_credentials()
                if not creds.get("gemini_api_key"):
                    logger.error("API Key configuration cancelled. Cannot run verification.")
                    return
            
            try:
                import oxide_core
            except ImportError:
                logger.error(f"Cannot JIT import oxide_core from {uniffi_path}")
                return
                
            board = pcbnew.GetBoard()
            success, message, violations = perform_board_clearance_sweep(board)
            
            root = tk.Tk()
            root.withdraw()
            if not success:
                messagebox.showerror("Oxide Tech Error", message, parent=root)
            else:
                if violations:
                    violation_summary = "\n".join([f"Component {v['designator']}: Pos({v['x']:.2f}, {v['y']:.2f}) H:{v['height']}mm" for v in violations])
                    ans = messagebox.askyesno(
                        "Clearance Violations Detected",
                        f"Detected {len(violations)} GJK Clearance Violations:\n\n{violation_summary}\n\nDo you want to spawn the AI eda-agent to run self-healing optimization?",
                        parent=root
                    )
                    if ans:
                        # Spawn eda-agent subprocess
                        board_path = board.GetFileName()
                        agent_success, agent_log = spawn_eda_agent(board_path)
                        if agent_success:
                            messagebox.showinfo("AI Self-Healing Success", f"EDA Agent successfully corrected layout layout parameters:\n\n{agent_log}", parent=root)
                        else:
                            messagebox.showerror("AI Self-Healing Failure", f"EDA Agent solver execution failed:\n\n{agent_log}", parent=root)
                else:
                    messagebox.showinfo("Clearance Clean", "All component heights and positions are compliant with enclosure casing clearance boundaries.", parent=root)
            root.destroy()

    # Register the action plugin inside KiCad environment
    OxideCoreVerifier().register()
    
except ImportError:
    pass


# MCP Tool Definition & Command-Line CLI handler
def mcp_tool_schema():
    """Returns the standardized MCP tool schema for kicad_verify_clearance."""
    return {
        "name": "kicad_verify_clearance",
        "description": "Performs physical enclosure clearance audits on active KiCad board designs using the native GJK engine solver.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "board_path": {
                    "type": "string",
                    "description": "Absolute path to the KiCad PCB document (.kicad_pcb)."
                }
            },
            "required": ["board_path"]
        }
    }

def handle_mcp_invocation(arguments):
    """Executes the GJK clearance sweep on a board file and returns standard response payload."""
    board_path = arguments.get("board_path")
    if not board_path or not os.path.exists(board_path):
        return {
            "isError": True,
            "content": [{"type": "text", "text": f"Error: Board file path does not exist: {board_path}"}]
        }
        
    try:
        import pcbnew
    except ImportError:
        return {
            "isError": True,
            "content": [{"type": "text", "text": "Error: pcbnew module not available in the current python interpreter."}]
        }
        
    try:
        board = pcbnew.LoadBoard(board_path)
        success, message, violations = perform_board_clearance_sweep(board)
        
        if not success:
            return {
                "isError": True,
                "content": [{"type": "text", "text": f"Clearance sweep failed: {message}"}]
            }
            
        result_payload = {
            "success": len(violations) == 0,
            "message": message,
            "violations": violations
        }
        
        return {
            "isError": False,
            "content": [{"type": "text", "text": json.dumps(result_payload, indent=2)}]
        }
    except Exception as e:
        return {
            "isError": True,
            "content": [{"type": "text", "text": f"Exception loading/processing board: {str(e)}"}]
        }


def run_mcp_stdio_server():
    """Runs a standard persistent Stdio JSON-RPC MCP server loop for eda-agent."""
    logger.info("KiCad-MCP Stdio Server loop running...")
    while True:
        try:
            line = sys.stdin.readline()
            if not line:
                break
                
            request = json.loads(line)
            method = request.get("method")
            req_id = request.get("id")
            params = request.get("params", {})
            
            if method == "tools/list":
                response = {
                    "jsonrpc": "2.0",
                    "result": {
                        "tools": [mcp_tool_schema()]
                    },
                    "id": req_id
                }
            elif method == "tools/call":
                name = params.get("name")
                arguments = params.get("arguments", {})
                if name == "kicad_verify_clearance":
                    result = handle_mcp_invocation(arguments)
                    response = {
                        "jsonrpc": "2.0",
                        "result": result,
                        "id": req_id
                    }
                else:
                    response = {
                        "jsonrpc": "2.0",
                        "error": {"code": -32601, "message": f"Method not found: {name}"},
                        "id": req_id
                    }
            else:
                response = {
                    "jsonrpc": "2.0",
                    "error": {"code": -32601, "message": f"Method not found: {method}"},
                    "id": req_id
                }
                
            sys.stdout.write(json.dumps(response) + "\n")
            sys.stdout.flush()
        except Exception as e:
            err_response = {
                "jsonrpc": "2.0",
                "error": {"code": -32603, "message": str(e)},
                "id": None
            }
            sys.stdout.write(json.dumps(err_response) + "\n")
            sys.stdout.flush()


if __name__ == "__main__":
    if len(sys.argv) > 1:
        cmd = sys.argv[1]
        if cmd == "--settings":
            dialog = ByokSettingsDialog()
            dialog.run()
        elif cmd == "--schema":
            print(json.dumps(mcp_tool_schema(), indent=2))
        elif cmd == "--mcp":
            run_mcp_stdio_server()
        elif cmd == "--call" and len(sys.argv) > 2:
            try:
                args = json.loads(sys.argv[2])
                response = handle_mcp_invocation(args)
                print(json.dumps(response, indent=2))
            except Exception as ex:
                print(json.dumps({"isError": True, "content": [{"type": "text", "text": f"Failed to parse arguments: {str(ex)}"}], "id": None}))
        else:
            print("Usage: python kicad_integration.py [--settings | --schema | --mcp | --call '<arguments_json>']")
    else:
        dialog = ByokSettingsDialog()
        dialog.run()
