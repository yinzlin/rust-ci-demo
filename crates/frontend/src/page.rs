use dioxus::prelude::*;
use ferris_says::say;

fn main() {
    launch(App);
}

#[component]
pub fn Home() -> Element {
    let message = "Hello from Dioxus Frontend!";

    rsx! {
        div {
            class: "container",
            h1 { "Welcome to Rust CI/CD Demo" }
            p { "Built with Dioxus + Axum" }
            div {
                class: "message-box",
                pre {
                    code { {message} }
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