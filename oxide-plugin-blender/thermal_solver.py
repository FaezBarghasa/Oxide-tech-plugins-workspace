import warp as wp
import numpy as np
import logging
from oxide_tech_client import OxideClient

logger = logging.getLogger("OxideThermalSolver")

class OxideThermalSolver:
    def __init__(self):
        wp.init()
        self.client = OxideClient()
    
    @wp.kernel
    def heat_equation_kernel(
        T: wp.array2d(dtype=wp.float32),
        T_new: wp.array2d(dtype=wp.float32),
        Q: wp.array2d(dtype=wp.float32),
        alpha: wp.float32,
        dx: wp.float32,
        dt: wp.float32,
    ):
        i, j = wp.tid()
        d2T_dx2 = (T[i+1, j] - 2.0 * T[i, j] + T[i-1, j]) / (dx * dx)
        d2T_dy2 = (T[i, j+1] - 2.0 * T[i, j] + T[i, j-1]) / (dx * dx)
        T_new[i, j] = T[i, j] + alpha * dt * (d2T_dx2 + d2T_dy2) + Q[i, j] * dt
    
    def solve_steady_state(self, board_dims: tuple, power_map: dict) -> wp.array2d:
        nx, ny = 100, 100
        dx = board_dims[0] / nx
        
        T = wp.zeros((nx, ny), dtype=wp.float32)
        T_new = wp.zeros((nx, ny), dtype=wp.float32)
        Q = self.build_power_map(power_map, nx, ny)
        
        alpha = 1.0e-4
        dt = 0.01
        
        for step in range(1000):
            wp.launch(
                kernel=self.heat_equation_kernel,
                dim=(nx-2, ny-2),
                inputs=[T, T_new, Q, alpha, dx, dt]
            )
            # Swap references
            temp = T
            T = T_new
            T_new = temp
        
        return T

    def build_power_map(self, power_map: dict, nx: int, ny: int) -> wp.array2d:
        """Convert power map dictionary (component_name -> power_w) to a 2D Warp array grid"""
        q_np = np.zeros((nx, ny), dtype=np.float32)
        try:
            placement = self.client.get_ai_placement("dummy_project")
            for ref_des, power in power_map.items():
                if ref_des in placement:
                    x, y, _ = placement[ref_des]
                    idx = int(max(0, min(nx - 1, (x / 100.0) * nx)))
                    idy = int(max(0, min(ny - 1, (y / 100.0) * ny)))
                    q_np[idx, idy] = float(power)
                else:
                    q_np[nx // 2, ny // 2] = float(power)
        except Exception as e:
            logger.warning(f"Failed to use placement for power mapping, falling back: {e}")
            for i, power in enumerate(power_map.values()):
                idx = (30 + i * 10) % nx
                idy = (30 + i * 10) % ny
                q_np[idx, idy] = float(power)
        
        return wp.array2d(q_np, dtype=wp.float32)
