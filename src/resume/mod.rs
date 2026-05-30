use dioxus::prelude::*;

mod data;

use data::{EXPERIENCE_POINTS, SKILLS};

#[component]
pub fn Resume() -> Element {
    rsx! {
        section { id: "resume", class: "section resume-section",
            div { class: "section-heading",
                p { class: "eyebrow", "Resume" }
                h2 { "Resume Summary" }
                p { "A concise version of my developer profile." }
            }

            div { class: "resume-layout",
                article { class: "card",
                    h3 { "Profile" }
                    p {
                        "Full-stack web developer with experience building practical web applications using React, Node.js, Express, MongoDB, Redis, and modern frontend/backend workflows."
                    }
                }

                article { class: "card",
                    h3 { "Experience Highlights" }
                    ul { class: "clean-list",
                        for point in EXPERIENCE_POINTS {
                            li { "{point}" }
                        }
                    }
                }

                article { class: "card",
                    h3 { "Skills" }
                    div { class: "tags",
                        for skill in SKILLS {
                            span { "{skill}" }
                        }
                    }
                }

                article { class: "card",
                    h3 { "Download Resume" }
                    p {
                        "Add your PDF resume later at assets/resume.pdf, then update this button to link to it."
                    }
                    a {
                        class: "button secondary",
                        href: "#contact",
                        "Contact Me"
                    }
                }
            }
        }
    }
}