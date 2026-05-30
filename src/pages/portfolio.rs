use crate::content::portfolio_content::PROJECTS;
use dioxus::prelude::*;

#[component]
pub fn Portfolio() -> Element {
    rsx! {
        main { class: "page-main",
            header { class: "page-header",
                p { class: "eyebrow", "Portfolio" }
                h1 { class: "page-title gradient-text", "Selected work" }
                p { class: "page-lead", "Projects are stored in src/content/portfolio_content.rs so the page content can grow without touching routing code." }
            }

            section { class: "project-grid",
                for project in PROJECTS {
                    article { class: "project-card",
                        p { class: "eyebrow", "{project.category}" }
                        h3 { "{project.title}" }
                        p { "{project.description}" }
                        div { class: "pill-row",
                            for tech in project.tech {
                                span { class: "pill", "{tech}" }
                            }
                        }
                        div { class: "card-actions",
                            a { class: "btn", href: "{project.link}", target: "_blank", rel: "noopener noreferrer", "View" }
                        }
                    }
                }
            }
        }
    }
}
