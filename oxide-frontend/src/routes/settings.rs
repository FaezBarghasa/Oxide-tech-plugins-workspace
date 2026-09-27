use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    rsx! {
        div {
            class: "flex flex-col h-full flex-1 bg-slate-800 items-center justify-center",
            h1 { class: "text-2xl text-gray-400", "Settings" }
        }
    }
}
