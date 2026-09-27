use dioxus::prelude::*;

#[component]
pub fn FileExplorer() -> Element {
    rsx! {
        div {
            class: "w-48 bg-slate-850 border-r border-slate-700 overflow-y-auto",
            div { class: "p-4",
                h3 { class: "font-bold mb-4", "Files" }
                FileList { }
            }
        }
    }
}

#[component]
fn FileList() -> Element {
    rsx! {
        ul {
            class: "space-y-1 text-sm",
            li { class: "px-2 py-1 hover:bg-slate-700 rounded cursor-pointer", "📁 src" }
            li { class: "px-2 py-1 hover:bg-slate-700 rounded cursor-pointer", "  📄 main.rs" }
            li { class: "px-2 py-1 hover:bg-slate-700 rounded cursor-pointer", "  📄 lib.rs" }
            li { class: "px-2 py-1 hover:bg-slate-700 rounded cursor-pointer", "📄 Cargo.toml" }
        }
    }
}
