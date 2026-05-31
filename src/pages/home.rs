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
                    p { class: "eyebrow", "Full-stack • SaaS • Web3 • E-commerce" }

                    h1 { class: "gradient-text", "Mohd Arafat Hossain" }

                    p {
                        "Full-stack Web3 & SaaS application developer with 5+ years of experience building practical web applications across payments, analytics, real-time features, APIs, exports, and business workflows."
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
                            p { class: "eyebrow", "Developer Profile" }

                            h2 { "I build complete web applications, not just isolated pages." }

                            p {
                                "My work spans frontend interfaces, backend APIs, databases, authentication, role-based access, smart contract integration, third-party API integration, payments, reports, real-time features, file exports, deployment and debugging."
                            }
                        }

                        div { class: "stat-grid",
                            div { class: "stat", strong { "5+" } span { "Years of professional working experience" } }
                            div { class: "stat", strong { "10+" } span { "Complete SaaS application delivered" } }
                            div { class: "stat", strong { "Quick!" } span { "I love moving fast and breaking things! - then fixing it." } }
                        }
                    }
                }
            }

            section { class: "section",
                div { class: "section-heading",
                    div {
                        p { class: "eyebrow", "Explore" }

                        h2 { "A portfolio for recruiters, teams, and technical collaborators." }

                        p { class: "page-lead",
                            "This site brings together my resume, project experience, and technical writing so people can quickly understand what I build, how I solve problems, and where I can contribute."
                        }
                    }
                }

                div { class: "card-grid",
                    div { class: "glass-card",
                        h3 { "Resume" }

                        p {
                            "A focused summary of my full-stack development experience, technical skills, backend/frontend work, and application-building background. I might just be the right person you are looking to hire!"
                        }

                        div { class: "card-actions",
                            Link { to: Route::Resume {}, class: "btn", "Open Resume" }
                        }
                    }

                    div { class: "glass-card",
                        h3 { "Portfolio" }

                        p {
                            "Projects across Web3, SaaS, e-commerce, admin dashboards, customer dashboards, payments, analytics, real-time features, and export-heavy workflows."
                        }

                        div { class: "card-actions",
                            Link { to: Route::Portfolio {}, class: "btn", "Open Portfolio" }
                        }
                    }

                    div { class: "glass-card",
                        h3 { "Blog" }

                        p {
                            "Technical writing about web development, backend systems, blockchain integration, debugging, system design, experiences, and software engineering decisions."
                        }

                        div { class: "card-actions",
                            Link { to: Route::Blog {}, class: "btn", "Open Blog" }
                        }
                    }
                }
            }

            section { id: "contact", class: "section contact-section",
                div { class: "section-heading",
                    div {
                        p { class: "eyebrow", "Contact" }

                        h2 { "Contact Me" }
                    }
                }

                div { class: "contact-card",
                    div {
                        h3 { "Start a focused conversation." }

                        p {
                            "Use one of the prepared contact options for hiring, project discussion, collaboration, or technical conversation."
                        }
                    }

                    div { class: "contact-links",
                        a {
                            class: "btn email-btn",
                            href: "https://mail.google.com/mail/?view=cm&fs=1&to=hmohdarafat@gmail.com&su=Full-stack%20Developer%20Opportunity&body=Hi%20Arafat%2C%0A%0AI%20saw%20your%20portfolio%20website%20and%20would%20like%20to%20discuss%20a%20full-stack%20developer%20opportunity.%0A%0ARole%3A%0ACompany%3A%0AWork%20type%20%28onsite%2Fremote%2Fhybrid%29%3A%0AExpected%20skills%3A%0A%0AThanks.",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            span { class: "email-label", "Hiring / Job" }
                            span { class: "email-address", "Email" }
                        }

                        a {
                            class: "btn email-btn",
                            href: "https://mail.google.com/mail/?view=cm&fs=1&to=hmohdarafat@gmail.com&su=Project%20Discussion&body=Hi%20Arafat%2C%0A%0AI%20saw%20your%20portfolio%20website%20and%20would%20like%20to%20discuss%20a%20web%20application%20project.%0A%0AProject%20type%3A%0ARequired%20features%3A%0ATimeline%3A%0ABudget%20range%3A%0A%0AThanks.",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            span { class: "email-label", "Project Discussion" }
                            span { class: "email-address", "Email" }
                        }

                        a {
                            class: "btn email-btn",
                            href: "https://mail.google.com/mail/?view=cm&fs=1&to=hmohdarafat@gmail.com&su=Web3%20%2F%20SaaS%20Collaboration&body=Hi%20Arafat%2C%0A%0AI%20saw%20your%20portfolio%20and%20would%20like%20to%20discuss%20a%20Web3%20or%20SaaS%20collaboration.%0A%0AProject%20or%20company%3A%0AWhat%20you%20are%20building%3A%0AHow%20I%20can%20help%3A%0A%0AThanks.",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            span { class: "email-label", "Web3 / SaaS" }
                            span { class: "email-address", "Email" }
                        }

                        a {
                            class: "btn email-btn",
                            href: "https://mail.google.com/mail/?view=cm&fs=1&to=hmohdarafat@gmail.com&su=Technical%20Discussion&body=Hi%20Arafat%2C%0A%0AI%20saw%20your%20technical%20writing%20and%20would%20like%20to%20discuss%20a%20software%20engineering%20topic.%0A%0ATopic%3A%0AQuestion%20or%20idea%3A%0A%0AThanks.",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            span { class: "email-label", "Technical Discussion" }
                            span { class: "email-address", "Email" }
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