use dioxus::prelude::*;
use crate::routes::*;
use crate::state::*;
use crate::components::layout::*;

#[component]
pub fn App() -> Element {
    // Initialize global state
    use_coroutine(|_: UnboundedReceiver<()>| async move {
        initialize_state().await;
    });

    rsx! {
        Router::<Route> { }
    }
}

#[derive(Routable, Clone)]
pub enum Route {
    #[layout(RootLayout)]
    #[route("/")]
    Home {},
    #[route("/editor")]
    Editor {},
    #[route("/pcb")]
    PCBViewer {},
    #[route("/schematic")]
    SchematicViewer {},
    #[route("/3d")]
    BlenderViewer {},
    #[route("/settings")]
    Settings {},
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
fn RootLayout() -> Element {
    rsx! {
        div {
            class: "flex flex-col h-screen w-screen bg-slate-900 text-white",
            Header { }
            div {
                class: "flex flex-1 overflow-hidden",
                Sidebar { }
                Outlet::<Route> { }
            }
            StatusBar { }
        }
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            class: "flex items-center justify-center h-full",
            h1 { "Page not found: /{route.join(\"/\")}" }
        }
    }
}

async fn initialize_state() {
    // Load user preferences from localStorage
    // Initialize WebSocket connection
    // Fetch recent projects
    tracing::info!("Application initialized");
}
