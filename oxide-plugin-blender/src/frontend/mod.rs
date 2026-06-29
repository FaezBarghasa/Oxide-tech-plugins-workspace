#[cfg(target_arch = "wasm32")]
pub mod state;
#[cfg(target_arch = "wasm32")]
pub mod components;

#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use state::{CADState, EnclosureParams};
#[cfg(target_arch = "wasm32")]
use components::MainLayout;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

#[cfg(target_arch = "wasm32")]
#[component]
pub fn App() -> Element {
    let params = use_signal(|| EnclosureParams::default());
    let history = use_signal(|| vec![EnclosureParams::default()]);
    let history_idx = use_signal(|| 0);
    let is_generating = use_signal(|| false);
    let error_msg = use_signal(|| None::<String>);
    let is_modal_open = use_signal(|| false);

    let mut state = CADState {
        params,
        history,
        history_idx,
        is_generating,
        error_msg,
        is_modal_open,
    };

    use_context_provider(|| state);

    // Fetch initial parameters from backend on mount
    use_effect(move || {
        spawn_local(async move {
            let client = reqwest::Client::new();
            if let Ok(resp) = client.get("/api/parameters").send().await {
                if resp.status().is_success() {
                    if let Ok(initial_params) = resp.json::<EnclosureParams>().await {
                        *state.params.write() = initial_params.clone();
                        *state.history.write() = vec![initial_params];
                        *state.history_idx.write() = 0;
                    }
                }
            }
        });
    });

    rsx! {
        style {
            r#"
            @keyframes bounce {{
              0%, 100% {{ transform: translateY(0); }}
              50% {{ transform: translateY(-4px); }}
            }}
            .dot-bounce {{
              display: inline-block;
              width: 4px;
              height: 4px;
              border-radius: 50%;
              background-color: #a1a1aa;
              animation: bounce 1.2s infinite ease-in-out;
            }}
            .dot-bounce:nth-child(2) {{ animation-delay: 0.2s; }}
            .dot-bounce:nth-child(3) {{ animation-delay: 0.4s; }}
            "#
        }
        MainLayout {}
    }
}
