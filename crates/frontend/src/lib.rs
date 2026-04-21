#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dioxus::prelude::*;
use dioxus_router::Routable;

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
}

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "container",
            h1 { "Welcome to Rust CI/CD Demo" }
            p { "Built with Dioxus + Axum" }
            div {
                class: "message-box",
                pre {
                    code { "Hello from Dioxus Frontend!" }
                }
            }
        }
    }
}

#[component]
pub fn About() -> Element {
    rsx! {
        div {
            h1 { "About" }
            p { "This is a full-stack Rust application using Dioxus and Axum." }
        }
    }
}

#[component]
pub fn App() -> Element {
    rsx! {
        dioxus_router::Router::<Route> {}
    }
}

pub fn main() {
    launch(App);
}
