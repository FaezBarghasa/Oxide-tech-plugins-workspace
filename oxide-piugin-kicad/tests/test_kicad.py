import unittest
from unittest.mock import MagicMock, patch
from schematic_generator import OxideSchematicGenerator, Schematic, ERCResults
from pcb_layout import OxidePCBLayout

class TestKiCadPlugin(unittest.TestCase):
    @patch('schematic_generator.OxideClient')
    def test_schematic_generator_generate(self, mock_client_cls):
        mock_client = MagicMock()
        mock_client_cls.return_value = mock_client
        mock_client.get_project_context.return_value = MagicMock()
        
        # Mock inference return value
        mock_inference_resp = MagicMock()
        mock_inference_resp.code = "from skidl import *\n"
        mock_client.inference.return_value = mock_inference_resp
        
        generator = OxideSchematicGenerator("test_project")
        generator.execute_skidl_sandbox = MagicMock(return_value=Schematic("test_netlist"))
        generator.run_erc = MagicMock(return_value=ERCResults([]))
        
        sch = generator.generate_from_ai("Create a resistor circuit")
        self.assertEqual(sch.netlist, "test_netlist")
        mock_client.sync_netlist_to_graph.assert_called_once_with("test_netlist")

    @patch('pcb_layout.pcbnew')
    @patch('pcb_layout.OxideClient')
    def test_pcb_layout_apply_placement(self, mock_client_cls, mock_pcbnew):
        mock_client = MagicMock()
        mock_client_cls.return_value = mock_client
        mock_client.get_ai_placement.return_value = {"R1": (10.0, 20.0, 90.0)}
        
        mock_board = MagicMock()
        mock_pcbnew.LoadBoard.return_value = mock_board
        mock_footprint = MagicMock()
        mock_board.FindFootprintByReference.return_value = mock_footprint
        
        layout = OxidePCBLayout("test_board.kicad_pcb", "test_project")
        layout.apply_ai_placement()
        
        mock_footprint.SetPosition.assert_called_once()
        mock_footprint.SetOrientation.assert_called_once_with(900)

if __name__ == '__main__':
    unittest.main()
