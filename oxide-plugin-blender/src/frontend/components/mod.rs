#[cfg(target_arch = "wasm32")]
mod layout;
#[cfg(target_arch = "wasm32")]
mod viewport;
#[cfg(target_arch = "wasm32")]
mod parametric;
#[cfg(target_arch = "wasm32")]
mod chat;

#[cfg(target_arch = "wasm32")]
pub use layout::MainLayout;
#[cfg(target_arch = "wasm32")]
pub use viewport::Scene;
#[cfg(target_arch = "wasm32")]
pub use parametric::ParametricInputPanel;
#[cfg(target_arch = "wasm32")]
pub use chat::ChatPanel;
