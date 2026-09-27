import unittest
from unittest.mock import MagicMock, patch
import sys
import os

# Add parent dir to sys.path to resolve local imports in headless tests
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from parametric_cad import OxideEnclosureGenerator
from thermal_solver import OxideThermalSolver

class TestBlenderPlugin(unittest.TestCase):
    @patch('parametric_cad.bpy')
    @patch('parametric_cad.OxideClient')
    def test_enclosure_generator(self, mock_client_cls, mock_bpy):
        mock_client = MagicMock()
        mock_client_cls.return_value = mock_client
        mock_ctx = MagicMock()
        mock_client.get_project_context.return_value = mock_ctx
        
        mock_pcb_dims = MagicMock()
        mock_pcb_dims.width = 100
        mock_pcb_dims.height = 80
        mock_ctx.get_pcb_dimensions.return_value = mock_pcb_dims
        mock_ctx.get_max_component_height.return_value = 10
        
        mock_connector = MagicMock()
        mock_connector.name = "USB"
        mock_connector.position = (0, 0, 0)
        mock_connector.dimensions = (5, 5, 5)
        mock_ctx.get_edge_connectors.return_value = [mock_connector]
        
        generator = OxideEnclosureGenerator("test_project")
        path = generator.generate_from_pcb()
        
        self.assertTrue(path.endswith("_enclosure.step"))
        mock_bpy.ops.mesh.primitive_cube_add.assert_called()

    @patch('thermal_solver.wp')
    @patch('thermal_solver.OxideClient')
    def test_thermal_solver(self, mock_client_cls, mock_wp):
        mock_client = MagicMock()
        mock_client_cls.return_value = mock_client
        mock_client.get_ai_placement.return_value = {"U1": (50.0, 50.0, 0.0)}
        
        solver = OxideThermalSolver()
        mock_wp.zeros.return_value = MagicMock()
        
        # Test steady state solve execution
        res = solver.solve_steady_state((100, 100), {"U1": 5.0})
        self.assertIsNotNone(res)
        mock_wp.launch.assert_called()

if __name__ == '__main__':
    unittest.main()
