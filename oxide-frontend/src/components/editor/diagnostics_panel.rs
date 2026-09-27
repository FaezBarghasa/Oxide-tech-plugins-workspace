use dioxus::prelude::*;
use crate::state::*;

#[component]
pub fn DiagnosticsPanel() -> Element {
    let compilation_state = use_signal(|| CompilationState::default());
    
    rsx! {
        div {
            class: "bg-slate-850 border-t border-slate-700 p-4 max-h-48 overflow-y-auto",
            h3 { class: "font-bold mb-2", "Diagnostics" }
            div {
                class: "space-y-2",
                for diag in &compilation_state.read().diagnostics {
                    DiagnosticItem { diagnostic: diag.clone() }
                }
            }
        }
    }
}

#[component]
fn DiagnosticItem(diagnostic: CompilationDiagnostic) -> Element {
    let level_color = match diagnostic.level.as_str() {
        "error" => "text-red-500",
        "warning" => "text-yellow-500",
        _ => "text-blue-500",
    };
    
    rsx! {
        div {
            class: "text-sm {level_color}",
            span { "{diagnostic.level}: " }
            span { "{diagnostic.message}" }
            if let Some(line) = diagnostic.line {
                span { class: "text-gray-400", " at line {line}" }
            }
        }
    }
}
