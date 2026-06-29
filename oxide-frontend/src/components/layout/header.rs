use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        div {
            class: "bg-slate-950 border-b border-slate-700 px-4 py-2 flex items-center justify-between",
            div {
                class: "flex items-center gap-4",
                h1 { class: "text-xl font-bold", "Oxide-Tech" }
                nav {
                    class: "flex gap-6",
                    a { href: "/", "Home" }
                    a { href: "/editor", "Editor" }
                    a { href: "/pcb", "PCB" }
                    a { href: "/schematic", "Schematic" }
                    a { href: "/3d", "3D" }
                }
            }
            div {
                class: "flex gap-2",
                button { class: "px-3 py-1 bg-blue-600 rounded hover:bg-blue-700", "Build" }
                button { class: "px-3 py-1 bg-gray-700 rounded hover:bg-gray-600", "Settings" }
            }
        }
    }
}
