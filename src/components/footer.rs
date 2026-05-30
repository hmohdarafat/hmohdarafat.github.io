use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "footer",
            p { "© 2026 Mohd Arafat Hossain. Built with Rust, Dioxus, and GitHub Pages." }
        }
    }
}