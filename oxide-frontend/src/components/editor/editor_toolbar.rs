use dioxus::prelude::*;
use crate::hooks::*;

#[component]
pub fn EditorToolbar() -> Element {
    let api = use_api();
    let mut is_compiling = use_signal(|| false);
    
    let handle_compile = move |_| {
        spawn(async move {
            *is_compiling.write() = true;
            // TODO: api.cargo_check().await
            *is_compiling.write() = false;
        });
    };

    rsx! {
        div {
            class: "flex gap-2 px-4 py-2 bg-slate-700 border-b border-slate-600",
            button {
                class: "px-3 py-1 bg-blue-600 rounded hover:bg-blue-700 disabled:opacity-50",
                onclick: handle_compile,
                disabled: *is_compiling.read(),
                if *is_compiling.read() { "Compiling..." } else { "Compile" }
            }
            button { class: "px-3 py-1 bg-gray-700 rounded hover:bg-gray-600", "Format" }
            button { class: "px-3 py-1 bg-gray-700 rounded hover:bg-gray-600", "Lint" }
            button { class: "px-3 py-1 bg-gray-700 rounded hover:bg-gray-600", "Save" }
        }
    }
}
