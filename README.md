# Oxide-tech-plugins-workspace

Upgraded plugin ecosystem connecting specialized engineering and CAD tools (KiCad, Altium, Blender, RustRover) to the Oxide-Tech graph database and AI inference engine.

## 📁 Workspace Structure

- **`oxide-piugin-kicad`**: Python Action Plugin and sync utilities for KiCad schematic generation and component layout checks.
- **`oxide-plugin-altium`**: C# and Rust modules for Altium Designer including IPC-2581 integration and Windows COM Automation.
- **`oxide-plugin-blender`**: Parametric enclosure generation and steady-state GPU thermal solver via NVIDIA Warp.
- **`oxide-plugin-rustrover`**: JetBrains IDE plugin for streaming AI inferences and triggering hardware verification MCP tools.
- **`shared`**: Rust shared types crate representing projects, tenants, and inference requests.

## 📖 Getting Started

Please see the comprehensive [USER_GUIDE.md](USER_GUIDE.md) for architecture, features, docker sandbox setups, and testing instructions.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
