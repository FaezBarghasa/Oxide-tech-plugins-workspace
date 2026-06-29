#![allow(non_snake_case)]

mod app;
mod components;
mod hooks;
mod services;
mod state;
mod types;
mod utils;
mod routes;

fn main() {
    dioxus::launch(app::App);
}
