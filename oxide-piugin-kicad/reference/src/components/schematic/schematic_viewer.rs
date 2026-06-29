use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn SchematicViewer(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Schematic Viewer"
        }
    })
}
