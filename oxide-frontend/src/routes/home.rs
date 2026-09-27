use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "flex-1 p-8",
            h1 { class: "text-3xl font-bold mb-4", "Welcome to Oxide-Tech Embedded Rust IDE" }
            p { class: "text-gray-400 mb-6", "Select a project to get started" }
            
            div {
                class: "grid grid-cols-3 gap-4",
                ProjectCard { name: "Firmware Project 1", mcu: "STM32H743" }
                ProjectCard { name: "Sensor Driver", mcu: "ESP32-C6" }
                ProjectCard { name: "Motor Controller", mcu: "RP2040" }
            }
        }
    }
}

#[component]
fn ProjectCard(name: String, mcu: String) -> Element {
    rsx! {
        div {
            class: "bg-slate-800 rounded-lg p-4 hover:bg-slate-700 cursor-pointer",
            h3 { class: "font-bold", "{name}" }
            p { class: "text-sm text-gray-400", "{mcu}" }
        }
    }
}
