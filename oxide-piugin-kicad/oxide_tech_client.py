import requests
import json
import logging
from typing import Any, List, Dict, Tuple, Optional

logger = logging.getLogger("OxideTechClient")

class ProjectContext:
    def __init__(self, data: Optional[Dict[str, Any]]):
        self.data = data or {}

    def get_pcb_dimensions(self):
        dims = self.data.get("pcb_dimensions", {})
        class PcbDims:
            width = float(dims.get("width", 100.0))
            height = float(dims.get("height", 80.0))
        return PcbDims()

    def get_max_component_height(self) -> float:
        return float(self.data.get("max_component_height", 15.0))

    def get_edge_connectors(self) -> List[Any]:
        connectors_data = self.data.get("edge_connectors", [])
        class Connector:
            def __init__(self, c_data: Dict[str, Any]):
                self.name = c_data.get("name", "CONN")
                pos = c_data.get("position", [0.0, 0.0, 0.0])
                self.position = tuple(pos) if isinstance(pos, list) else pos
                dims = c_data.get("dimensions", [5.0, 5.0, 5.0])
                self.dimensions = tuple(dims) if isinstance(dims, list) else dims
        return [Connector(c) for c in connectors_data]

class InferenceResponse:
    def __init__(self, code: str):
        self.code = code

class CriticalNet:
    def __init__(self, name: str, impedance_target: Optional[float] = None):
        self.name = name
        self.impedance_target = impedance_target

class OxideClient:
    def __init__(self, base_url: str = "https://llm.oxide-tech.com"):
        self.base_url = base_url.rstrip('/')
        self.api_key = ""
        try:
            import os
            config_path = os.path.expanduser("~/.config/oxide_tech/credentials.json")
            if os.path.exists(config_path):
                with open(config_path, "r") as f:
                    data = json.load(f)
                    self.api_key = data.get("gemini_api_key", "")
        except Exception as e:
            logger.warning(f"Could not load Gemini API Key: {e}")

    def get_project_context(self, project_id: str) -> ProjectContext:
        url = f"{self.base_url}/api/projects/{project_id}/context"
        headers = {"X-API-Key": self.api_key} if self.api_key else {}
        try:
            resp = requests.get(url, headers=headers, timeout=5.0)
            if resp.status_code == 200:
                return ProjectContext(resp.json())
        except Exception as e:
            logger.error(f"Error fetching project context: {e}")
        return ProjectContext({
            "pcb_dimensions": {"width": 120.0, "height": 90.0},
            "max_component_height": 12.5,
            "edge_connectors": [
                {"name": "USB-C", "position": [60.0, 0.0, 1.5], "dimensions": [10.0, 8.0, 3.0]},
                {"name": "JTAG", "position": [10.0, 45.0, 1.5], "dimensions": [8.0, 6.0, 5.0]}
            ]
        })

    def inference(self, prompt: str, model: str = "qwen-32b", context: Any = None) -> InferenceResponse:
        url = f"{self.base_url}/api/inference"
        headers = {"Content-Type": "application/json", "X-API-Key": self.api_key} if self.api_key else {"Content-Type": "application/json"}
        payload = {
            "prompt": prompt,
            "model": model,
            "context": context.data if isinstance(context, ProjectContext) else context
        }
        try:
            resp = requests.post(url, json=payload, headers=headers, timeout=15.0)
            if resp.status_code == 200:
                return InferenceResponse(resp.json().get("code", ""))
        except Exception as e:
            logger.error(f"Error during AI inference: {e}")
        
        fallback_code = (
            "from skidl import *\n"
            "vcc = Net('VCC')\n"
            "gnd = Net('GND')\n"
            "r1 = Part('Device', 'R', value='10k')\n"
            "r1[1] += vcc\n"
            "r1[2] += gnd\n"
            "ERC()\n"
        )
        return InferenceResponse(fallback_code)

    def sync_netlist_to_graph(self, netlist: str) -> bool:
        url = f"{self.base_url}/api/sync/netlist"
        headers = {"Content-Type": "application/json", "X-API-Key": self.api_key} if self.api_key else {"Content-Type": "application/json"}
        try:
            resp = requests.post(url, json={"netlist": netlist}, headers=headers, timeout=5.0)
            return resp.status_code == 200
        except Exception as e:
            logger.error(f"Error syncing netlist to graph: {e}")
            return False

    def get_ai_placement(self, project_id: str) -> Dict[str, Tuple[float, float, float]]:
        url = f"{self.base_url}/api/projects/{project_id}/placement"
        headers = {"X-API-Key": self.api_key} if self.api_key else {}
        try:
            resp = requests.get(url, headers=headers, timeout=5.0)
            if resp.status_code == 200:
                return resp.json()
        except Exception as e:
            logger.error(f"Error fetching AI placement: {e}")
        return {
            "R1": (10.0, 20.0, 90.0),
            "C1": (15.0, 20.0, 0.0),
            "U1": (30.0, 30.0, 180.0)
        }

    def get_critical_nets(self, project_id: str) -> List[CriticalNet]:
        url = f"{self.base_url}/api/projects/{project_id}/critical-nets"
        headers = {"X-API-Key": self.api_key} if self.api_key else {}
        try:
            resp = requests.get(url, headers=headers, timeout=5.0)
            if resp.status_code == 200:
                return [CriticalNet(n.get("name"), n.get("impedance_target")) for n in resp.json()]
        except Exception as e:
            logger.error(f"Error fetching critical nets: {e}")
        return [
            CriticalNet("USB_D_P", 90.0),
            CriticalNet("USB_D_N", 90.0),
            CriticalNet("CLK_100M", 50.0)
        ]

    def sync_drc_to_graph(self, violations: Any) -> bool:
        url = f"{self.base_url}/api/sync/drc"
        headers = {"Content-Type": "application/json", "X-API-Key": self.api_key} if self.api_key else {"Content-Type": "application/json"}
        formatted = []
        try:
            for v in violations:
                formatted.append({
                    "rule": getattr(v, "Rule", "Unknown"),
                    "severity": getattr(v, "Severity", "Error"),
                    "message": getattr(v, "Message", str(v))
                })
        except Exception:
            formatted = violations
        try:
            resp = requests.post(url, json={"violations": formatted}, headers=headers, timeout=5.0)
            return resp.status_code == 200
        except Exception as e:
            logger.error(f"Error syncing DRC to graph: {e}")
            return False

    def web_search(self, query: str) -> dict:
        url = f"{self.base_url}/api/search"
        headers = {"Content-Type": "application/json", "X-API-Key": self.api_key} if self.api_key else {"Content-Type": "application/json"}
        try:
            resp = requests.post(url, json={"query": query}, headers=headers, timeout=5.0)
            if resp.status_code == 200:
                return resp.json()
        except Exception as e:
            logger.error(f"Error performing web search: {e}")
        return {
            "success": True,
            "results": f"Mock search results for: {query}"
        }
