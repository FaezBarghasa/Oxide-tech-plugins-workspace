use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn ComponentLibraryPanel(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Component Library Panel"
        }
    })
}
