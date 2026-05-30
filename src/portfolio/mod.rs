use dioxus::prelude::*;

mod data;

use data::PROJECTS;

#[component]
pub fn Portfolio() -> Element {
    rsx! {
        section { id: "portfolio", class: "section",
            div { class: "section-heading",
                p { class: "eyebrow", "Portfolio" }
                h2 { "Selected Projects" }
                p { "Projects that show backend, frontend, architecture, and learning depth." }
            }

            div { class: "grid cards",
                for project in PROJECTS {
                    article { class: "card",
                        h3 { "{project.title}" }
                        p { "{project.description}" }
                        div { class: "tags",
                            for tech in project.stack {
                                span { "{tech}" }
                            }
                        }
                        a {
                            class: "text-link",
                            href: "{project.href}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "View link →"
                        }
                    }
                }
            }
        }
    }
}