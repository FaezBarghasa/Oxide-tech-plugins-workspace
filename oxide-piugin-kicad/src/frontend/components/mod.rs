#[cfg(target_arch = "wasm32")]
pub mod layout;
#[cfg(target_arch = "wasm32")]
pub mod designer;
#[cfg(target_arch = "wasm32")]
pub mod viewers;
#[cfg(target_arch = "wasm32")]
pub mod chat;
#[cfg(target_arch = "wasm32")]
pub mod library;

#[cfg(target_arch = "wasm32")]
pub use layout::MainLayout;
#[cfg(target_arch = "wasm32")]
pub use designer::ComponentDesigner;
#[cfg(target_arch = "wasm32")]
pub use viewers::{SchematicViewer, PCBViewer};
#[cfg(target_arch = "wasm32")]
pub use chat::ChatPanel;
#[cfg(target_arch = "wasm32")]
pub use library::ComponentLibraryPanel;
