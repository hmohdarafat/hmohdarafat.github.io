use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    rsx! {
        section { id: "contact", class: "section contact-section",
            div { class: "section-heading",
                p { class: "eyebrow", "Contact" }
                h2 { "Contact Me" }
                p { "For work, collaboration, or technical discussion." }
            }

            div { class: "contact-card",
                a { href: "mailto:hmohdarafat@gmail.com", "hmohdarafat@gmail.com" }
                a {
                    href: "https://github.com/hmohdarafat",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "GitHub"
                }
                a {
                    href: "https://www.linkedin.com/in/hmohdarafat/",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "LinkedIn"
                }
            }
        }
    }
}