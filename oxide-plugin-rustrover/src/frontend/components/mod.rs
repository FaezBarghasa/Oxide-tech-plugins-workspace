#[cfg(target_arch = "wasm32")]
mod layout;
#[cfg(target_arch = "wasm32")]
mod skills;
#[cfg(target_arch = "wasm32")]
mod mcp;
#[cfg(target_arch = "wasm32")]
mod jetbrains;
#[cfg(target_arch = "wasm32")]
mod review;
#[cfg(target_arch = "wasm32")]
mod chat;
#[cfg(target_arch = "wasm32")]
mod explorer;

#[cfg(target_arch = "wasm32")]
pub use layout::MainLayout;
#[cfg(target_arch = "wasm32")]
pub use skills::SkillsPanel;
#[cfg(target_arch = "wasm32")]
pub use mcp::McpPanel;
#[cfg(target_arch = "wasm32")]
pub use jetbrains::JetbrainsPanel;
#[cfg(target_arch = "wasm32")]
pub use review::ReviewPanel;
#[cfg(target_arch = "wasm32")]
pub use chat::ChatPanel;
#[cfg(target_arch = "wasm32")]
pub use explorer::ExplorerPanel;
