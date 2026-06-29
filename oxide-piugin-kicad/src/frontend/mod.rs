#[cfg(target_arch = "wasm32")]
pub mod state;
#[cfg(target_arch = "wasm32")]
pub mod components;

#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use state::{KiCadState, SchematicData, PCBData, PCBBounds};
#[cfg(target_arch = "wasm32")]
use components::MainLayout;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

#[cfg(target_arch = "wasm32")]
#[component]
pub fn App() -> Element {
    let schematic = use_signal(|| SchematicData {
        components: vec![],
        nets: vec![],
    });

    let pcb = use_signal(|| PCBData {
        board_name: "".to_string(),
        footprints: vec![],
        traces: vec![],
        board_bounds: PCBBounds {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 120.0,
            max_y: 120.0,
        },
    });

    let board_sync = use_signal(|| None);
    let drc_report = use_signal(|| None);
    let active_panel = use_signal(|| "designer".to_string());
    let selected_ref = use_signal(|| None);
    let search_query = use_signal(|| "".to_string());
    let is_loading = use_signal(|| false);
    let error_msg = use_signal(|| None::<String>);
    
    let visible_layers = use_signal(|| {
        let mut hs = std::collections::HashSet::new();
        hs.insert("F.Cu".to_string());
        hs.insert("F.Silkscreen".to_string());
        hs.insert("B.Cu".to_string());
        hs
    });

    let mut state = KiCadState {
        schematic,
        pcb,
        board_sync,
        drc_report,
        active_panel,
        selected_ref,
        search_query,
        is_loading,
        error_msg,
        visible_layers,
    };

    use_context_provider(|| state);

    // Fetch initial parameters from backend on mount
    use_effect(move || {
        spawn_local(async move {
            let client = reqwest::Client::new();
            
            // 1. Fetch Schematic
            if let Ok(resp) = client.get("/api/schematic").send().await {
                if resp.status().is_success() {
                    if let Ok(initial_sch) = resp.json::<SchematicData>().await {
                        state.schematic.set(initial_sch);
                    }
                }
            }

            // 2. Fetch PCB Layout
            if let Ok(resp) = client.get("/api/pcb").send().await {
                if resp.status().is_success() {
                    if let Ok(initial_pcb) = resp.json::<PCBData>().await {
                        state.pcb.set(initial_pcb);
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
              background-color: #14b8a6;
              animation: bounce 1.2s infinite ease-in-out;
            }}
            .dot-bounce:nth-child(2) {{ animation-delay: 0.2s; }}
            .dot-bounce:nth-child(3) {{ animation-delay: 0.4s; }}
            "#
        }
        MainLayout {}
    }
}
