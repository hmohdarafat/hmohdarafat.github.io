use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    use_effect(move || {
        document::eval(
            r#"
            const shouldScrollToContact =
                window.location.hash === '#contact' ||
                sessionStorage.getItem('scrollToContact') === 'true';

            if (shouldScrollToContact) {
                let attempts = 0;

                const finishJump = () => {
                    sessionStorage.removeItem('scrollToContact');
                    document.documentElement.classList.remove('contact-jump');
                };

                const scrollToContact = () => {
                    const contactSection = document.getElementById('contact');
                    attempts += 1;

                    if (contactSection) {
                        window.history.replaceState(null, '', '/#contact');

                        contactSection.scrollIntoView({
                            behavior: 'auto',
                            block: 'start'
                        });

                        requestAnimationFrame(finishJump);
                    } else if (attempts < 20) {
                        requestAnimationFrame(scrollToContact);
                    } else {
                        finishJump();
                    }
                };

            requestAnimationFrame(scrollToContact);
        }
        "#,
    );
});

    rsx! {
        main { class: "page-main",
            section { class: "hero",
                div {
                    p { class: "eyebrow", "Rust • Full-stack • Systems" }
                    h1 { class: "gradient-text", "Mohd Arafat Hossain" }
                    p {
                        "A focused personal website for resume, portfolio projects, and technical writing about Rust, web engineering, and system design."
                    }
                    div { class: "hero-actions",
                        Link { to: Route::Resume {}, class: "btn primary", "View Resume" }
                        Link { to: Route::Portfolio {}, class: "btn", "See Portfolio" }
                        Link { to: Route::Blog {}, class: "btn", "Read Blog" }
                    }
                }

                div { class: "hero-card",
                    div { class: "hero-card-inner",
                        div {
                            p { class: "eyebrow", "Current focus" }
                            h2 { "Building practical software and documenting the engineering decisions behind it." }
                            p {
                                "This homepage is intentionally routed into separate sections so the project stays manageable as the resume, projects, and blog grow."
                            }
                        }
                        div { class: "stat-grid",
                            div { class: "stat", strong { "3" } span { "Main sections" } }
                            div { class: "stat", strong { "10" } span { "Latest posts" } }
                            div { class: "stat", strong { "100%" } span { "Rust/WASM" } }
                        }
                    }
                }
            }

            section { class: "section",
                div { class: "section-heading",
                    div {
                        p { class: "eyebrow", "Explore" }
                        h2 { "One site. Separate content files." }
                    }
                }
                div { class: "card-grid",
                    div { class: "glass-card",
                        h3 { "Resume" }
                        p { "A dedicated resume page with summary, skills, and experience pulled from one Rust content file." }
                        div { class: "card-actions", Link { to: Route::Resume {}, class: "btn", "Open Resume" } }
                    }
                    div { class: "glass-card",
                        h3 { "Portfolio" }
                        p { "Project cards are managed from one portfolio content file so updates stay simple." }
                        div { class: "card-actions", Link { to: Route::Portfolio {}, class: "btn", "Open Portfolio" } }
                    }
                    div { class: "glass-card",
                        h3 { "Blog" }
                        p { "Each post lives in its own file. The blog index shows the latest ten posts automatically." }
                        div { class: "card-actions", Link { to: Route::Blog {}, class: "btn", "Open Blog" } }
                    }
                }
            }

            section { id: "contact", class: "section contact-section",
                div { class: "section-heading",
                    div {
                        p { class: "eyebrow", "Contact" }
                        h2 { "Contact Me" }
                        p { class: "page-lead", "For work, collaboration, or technical discussion." }
                    }
                }

                div { class: "contact-card",
                    div {
                        h3 { "Let’s build something useful." }
                        p { "Email is the best first contact point." }
                    }
                    div { class: "contact-links",
                        a {
                            class: "btn email-btn",
                            href: "https://mail.google.com/mail/?view=cm&fs=1&to=hmohdarafat@gmail.com",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            span { class: "email-label", "Email" }
                            span { class: "email-address", "hmohdarafat@gmail.com" }
                        }

                        a {
                            class: "btn",
                            href: "https://github.com/hmohdarafat",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "GitHub"
                        }

                        a {
                            class: "btn",
                            href: "https://www.linkedin.com/in/mohdarafathossain/",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "LinkedIn"
                        }
                    }
                }
            }
        }
    }
}