#![allow(non_snake_case)]

use dioxus::prelude::*;

mod components;

use components::layout::main_layout::MainLayout;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        MainLayout {}
    })
}
