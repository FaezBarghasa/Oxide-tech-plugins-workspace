use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn ComponentDesigner(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Component Designer"
        }
    })
}
