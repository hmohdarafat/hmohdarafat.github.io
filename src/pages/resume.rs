use crate::content::resume_content::{EDUCATION, EXPERIENCE, RESUME, SKILLS};
use dioxus::prelude::*;

#[component]
pub fn Resume() -> Element {
    rsx! {
        main { class: "page-main",
            header { class: "page-header",
                p { class: "eyebrow", "Resume" }
                h1 { class: "page-title gradient-text", "{RESUME.name}" }
                p { class: "page-lead", "{RESUME.headline}" }
                div { class: "hero-actions",
                    a { class: "btn primary", href: "mailto:{RESUME.email}", "Email Me" }
                    a { class: "btn", href: "{RESUME.github}", target: "_blank", rel: "noopener noreferrer", "GitHub" }
                    a { class: "btn", href: "{RESUME.linkedin}", target: "_blank", rel: "noopener noreferrer", "LinkedIn" }
                }
            }

            div { class: "resume-grid",
                aside { class: "resume-section",
                    h2 { "Summary" }
                    p { "{RESUME.summary}" }
                    h2 { "Skills" }
                    div { class: "pill-row",
                        for skill in SKILLS {
                            span { class: "pill", "{skill}" }
                        }
                    }
                }

                section { class: "resume-section",
                    h2 { "Experience" }
                    div { class: "timeline",
                        for item in EXPERIENCE {
                            div { class: "timeline-item",
                                h3 { "{item.role}" }
                                p { class: "meta", "{item.company} • {item.period}" }
                                p { "{item.details}" }
                            }
                        }
                    }

                    h2 { "Education" }
                    div { class: "timeline",
                        for item in EDUCATION {
                            div { class: "timeline-item",
                                h3 { "{item.degree}" }
                                p { class: "meta", "{item.school} • {item.period}" }
                                p { "{item.details}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
