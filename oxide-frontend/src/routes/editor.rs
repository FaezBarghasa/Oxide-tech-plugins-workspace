use dioxus::prelude::*;
use crate::components::editor::*;
use crate::state::*;

#[component]
pub fn Editor() -> Element {
    let mut editor_state = use_signal(EditorState::default);
    
    // Set initial content mock
    if editor_state.read().content.is_empty() {
        editor_state.write().content = r#"// STM32 Breakout Board Firmware
fn main() {
    setup_gpio();
    loop {
        toggle_led();
        delay(1000);
    }
}

pub struct Config {
    pin: u8,
    mode: Mode,
}

pub enum Mode {
    Input,
    Output,
}

pub trait Device {
    fn init(&mut self);
}

impl Device for Config {
    fn init(&mut self) {
        // init
    }
}
"#.to_string();
    }
    
    rsx! {
        div {
            class: "flex flex-col h-full flex-1 bg-slate-800",
            EditorToolbar { }
            div {
                class: "flex flex-1 overflow-hidden",
                FileExplorer { }
                CodeEditor { editor_state }
                AstViewer { content: editor_state.read().content.clone() }
            }
            DiagnosticsPanel { }
            Terminal { }
        }
    }
}
