use dioxus::prelude::*;

#[component]
pub fn Sidebar() -> Element {
    let mut expanded = use_signal(|| true);

    rsx! {
        aside {
            class: "w-64 bg-slate-850 border-r border-slate-700 overflow-y-auto",
            div { class: "p-4",
                h2 { class: "font-bold mb-4", "Explorer" }
                FileTree { }
            }
        }
    }
}

#[component]
fn FileTree() -> Element {
    rsx! {
        ul {
            class: "space-y-1",
            FileTreeItem { name: "src", is_folder: true }
            FileTreeItem { name: "Cargo.toml", is_folder: false }
            FileTreeItem { name: ".gitignore", is_folder: false }
        }
    }
}

#[component]
fn FileTreeItem(name: String, is_folder: bool) -> Element {
    let icon = if is_folder { "📁" } else { "📄" };
    
    rsx! {
        li {
            class: "px-2 py-1 hover:bg-slate-700 rounded cursor-pointer",
            "{icon} {name}"
        }
    }
}
