use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { id: "home", class: "hero section",
            div { class: "hero-content",
                p { class: "eyebrow", "Full-Stack Developer • Rust Learner • System Design Focused" }
                h1 { "Building practical web systems with clean architecture." }
                p { class: "hero-text",
                    "I am Mohd Arafat Hossain, a full-stack developer focused on React, Node.js, MongoDB, Redis, Linux, Rust, and system design."
                }
                div { class: "hero-actions",
                    a { class: "button primary", href: "#portfolio", "View Projects" }
                    a { class: "button secondary", href: "#resume", "View Resume" }
                }
            }
            div { class: "hero-card",
                p { class: "card-label", "Current Focus" }
                h2 { "Interview-ready engineering depth" }
                p {
                    "System design, backend architecture, Rust, Linux, and production-style debugging."
                }
            }
        }
    }
}