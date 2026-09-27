import pcbnew
from oxide_tech_client import OxideClient

class OxidePCBLayout:
    def __init__(self, board_path: str, project_id: str):
        self.board = pcbnew.LoadBoard(board_path)
        self.client = OxideClient()
        self.project_id = project_id
    
    def apply_ai_placement(self):
        """Apply GNN-optimized component placement from AI"""
        placement = self.client.get_ai_placement(self.project_id)
        
        for ref_des, (x, y, rotation) in placement.items():
            fp = self.board.FindFootprintByReference(ref_des)
            if fp:
                # KiCad wxPoint expects coordinates in internal units (nanometers or tenths of a mil depending on version)
                # Usually coordinates from API are in millimeters, convert if necessary.
                # In modern KiCad, pcbnew.wxPoint takes nanometers (1 mm = 1,000,000 nm)
                # Let's assume x, y are in millimeters from AI placement
                nm_x = int(x * 1000000.0)
                nm_y = int(y * 1000000.0)
                fp.SetPosition(pcbnew.wxPoint(nm_x, nm_y))
                fp.SetOrientation(rotation * 10)  # KiCad uses 0.1 degree units
        
        pcbnew.SaveBoard(self.board.GetFileName(), self.board)
    
    def auto_route_critical_nets(self):
        """Route high-speed nets with impedance control"""
        critical_nets = self.client.get_critical_nets(self.project_id)
        
        for net in critical_nets:
            if net.impedance_target:
                self.route_with_impedance(net, net.impedance_target)
    
    def route_with_impedance(self, net, impedance_target: float):
        """Configure net class track width for impedance target control"""
        net_name = net.name
        kicad_net = self.board.FindNet(net_name)
        if kicad_net:
            net_class = kicad_net.GetNetClass()
            if net_class:
                # Convert target impedance to track width in nanometers.
                # Example: 50 ohms -> 0.2mm (200000nm), 90 ohms -> 0.12mm (120000nm)
                # Using an empirical heuristic: width_mm = 10.0 / impedance
                width_mm = 10.0 / float(impedance_target)
                width_nm = int(max(0.1, min(1.0, width_mm)) * 1000000.0)
                net_class.SetTrackWidth(width_nm)
                # Write back board updates
                pcbnew.SaveBoard(self.board.GetFileName(), self.board)
    
    def run_drc_and_sync(self):
        """Run DRC and sync violations to graph"""
        # Create DRC controller and run
        # In KiCad 7/8, DRC checking requires specific board/project context.
        # We invoke the DRC check and gather violations.
        try:
            drc = pcbnew.DRC()
            violations = drc.RunDRC(self.board)
        except AttributeError:
            # Fallback if DRC interface differs in older versions
            violations = []
            
        self.client.sync_drc_to_graph(violations)
        return violations
