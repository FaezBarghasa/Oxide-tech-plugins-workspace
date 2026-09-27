use dioxus::prelude::*;
use crate::utils::ast_parser::{AstNode, parse_rust_ast};

#[component]
pub fn AstViewer(content: String) -> Element {
    let nodes = parse_rust_ast(&content);

    rsx! {
        div {
            class: "w-64 bg-slate-850 border-l border-slate-700 flex flex-col h-full overflow-hidden",
            div {
                class: "px-4 py-2 bg-slate-800 border-b border-slate-700 flex items-center justify-between",
                span { class: "text-sm font-semibold text-gray-200", "AST / Outline" }
                span { class: "text-xs text-gray-500", "{nodes.len()} symbols" }
            }
            div {
                class: "flex-1 p-2 overflow-y-auto font-mono text-xs space-y-1",
                if nodes.is_empty() {
                    div { class: "text-gray-500 italic p-2 text-center", "No signatures parsed" }
                } else {
                    for node in nodes {
                        AstNodeItem { node }
                    }
                }
            }
        }
    }
}

#[component]
fn AstNodeItem(node: AstNode) -> Element {
    let icon = match node.kind.as_str() {
        "Function" => "ƒ",
        "Struct" => "S",
        "Enum" => "E",
        "Trait" => "T",
        "ImplBlock" => "I",
        _ => "•",
    };

    let badge_color = match node.kind.as_str() {
        "Function" => "text-emerald-400 bg-emerald-950/40 border border-emerald-800/40",
        "Struct" => "text-sky-400 bg-sky-950/40 border border-sky-800/40",
        "Enum" => "text-purple-400 bg-purple-950/40 border border-purple-800/40",
        "Trait" => "text-amber-400 bg-amber-950/40 border border-amber-800/40",
        "ImplBlock" => "text-indigo-400 bg-indigo-950/40 border border-indigo-800/40",
        _ => "text-gray-400 bg-gray-950/40 border border-gray-800/40",
    };

    rsx! {
        div {
            class: "flex items-center gap-2 p-1.5 hover:bg-slate-800 rounded cursor-pointer transition-colors group",
            span {
                class: "w-5 h-5 flex items-center justify-center text-[10px] font-bold rounded {badge_color}",
                "{icon}"
            }
            div {
                class: "flex flex-col min-w-0 flex-1",
                span { class: "text-gray-300 font-medium truncate group-hover:text-white", "{node.name}" }
                span { class: "text-[9px] text-gray-500", "Line {node.line}" }
            }
        }
    }
}
