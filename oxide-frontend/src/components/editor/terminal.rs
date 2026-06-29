use dioxus::prelude::*;

#[component]
pub fn Terminal() -> Element {
    let output = use_signal(Vec::<String>::new);
    
    rsx! {
        div {
            class: "h-32 bg-slate-900 border-t border-slate-700 p-4 font-mono text-sm overflow-y-auto",
            for line in output.read().iter() {
                p { class: "text-gray-300", "{line}" }
            }
        }
    }
}
