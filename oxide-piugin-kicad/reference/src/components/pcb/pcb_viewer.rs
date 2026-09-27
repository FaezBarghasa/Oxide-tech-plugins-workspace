use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn PCBViewer(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "PCB Viewer"
        }
    })
}
