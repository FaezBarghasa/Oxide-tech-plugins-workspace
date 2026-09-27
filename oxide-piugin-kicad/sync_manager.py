import asyncio
import websockets
import json
import pcbnew

class KiCadSyncManager:
    def __init__(self, board_path: str, project_id: str):
        self.board_path = board_path
        self.project_id = project_id
        self.ws_url = f"wss://llm.oxide-tech.com/ws/kicad/{project_id}"
    
    async def start(self):
        """Bi-directional sync with SurrealDB graph"""
        async with websockets.connect(self.ws_url) as ws:
            # Subscribe to graph changes
            await ws.send(json.dumps({
                "type": "subscribe",
                "channel": f"project:{self.project_id}:graph"
            }))
            
            # Listen for updates
            async for msg in ws:
                event = json.loads(msg)
                if event["type"] == "graph_updated":
                    await self.apply_graph_changes(event["changes"])
    
    async def apply_graph_changes(self, changes):
        """Apply graph changes to KiCad board"""
        board = pcbnew.LoadBoard(self.board_path)
        
        for change in changes:
            if change["type"] == "component_added":
                self.add_footprint(board, change["data"])
            elif change["type"] == "net_changed":
                self.update_net(board, change["data"])
        
        pcbnew.SaveBoard(self.board_path, board)

    def add_footprint(self, board, data):
        """Add footprint to KiCad board"""
        ref = data.get("reference")
        lib = data.get("library", "Device")
        fp_name = data.get("footprint", "R_0603_1608Metric")
        x = data.get("x", 0.0)
        y = data.get("y", 0.0)
        
        try:
            # Load footprint from library
            fp = pcbnew.FootprintLoad(lib, fp_name)
            if fp:
                fp.SetReference(ref)
                # Position in nanometers (assume x, y are in millimeters)
                fp.SetPosition(pcbnew.wxPoint(int(x * 1000000.0), int(y * 1000000.0)))
                board.Add(fp)
        except Exception as e:
            print(f"Error adding footprint {ref}: {e}")
            
    def update_net(self, board, data):
        """Update Net properties or add track connectivity"""
        net_name = data.get("name")
        if not net_name:
            return
            
        try:
            # Ensure Net exists in the board
            net = board.FindNet(net_name)
            if not net:
                net = pcbnew.NETINFO_ITEM(board, net_name)
                board.Add(net)
        except Exception as e:
            print(f"Error updating net {net_name}: {e}")
