use dioxus::prelude::*;
use crate::state::*;

#[component]
pub fn StatusBar() -> Element {
    let compilation_state = use_signal(|| CompilationState::default());
    
    rsx! {
        div {
            class: "bg-slate-950 border-t border-slate-700 px-4 py-2 text-sm text-gray-400 flex justify-between",
            div {
                "Ready • UTF-8 • Rust • Line: 0, Col: 0"
            }
            div {
                "Compilation: {compilation_state.read().status}"
            }
        }
    }
}
