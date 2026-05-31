use crate::content::resume_content::{
    EDUCATION, EXPERIENCE, PROJECTS, RESUME, SKILL_GROUPS,
};
use dioxus::prelude::*;

#[component]
pub fn Resume() -> Element {
    rsx! {
        main { class: "page-main",
            header { class: "page-header",
                p { class: "eyebrow", "Resume" }
                h1 { class: "page-title gradient-text", "{RESUME.name}" }
                p { class: "page-lead", "{RESUME.headline}" }

                p { class: "meta", "{RESUME.location}" }

                div { class: "hero-actions",
                    a {
                        class: "btn email-btn",
                        href: "https://mail.google.com/mail/?view=cm&fs=1&to={RESUME.email}&su=Full-stack%20Developer%20Opportunity&body=Hi%20Arafat%2C%0A%0AI%20saw%20your%20resume%20website%20and%20would%20like%20to%20discuss%20a%20full-stack%20developer%20opportunity.%0A%0ARole%3A%0ACompany%3A%0AWork%20type%3A%0AExpected%20skills%3A%0A%0AThanks.",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        span { class: "email-label", "Email" }
                        span { class: "email-address", "{RESUME.email}" }
                    }

                    a {
                        class: "btn",
                        href: "{RESUME.github}",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "GitHub"
                    }

                    a {
                        class: "btn",
                        href: "{RESUME.linkedin}",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "LinkedIn"
                    }

                    a {
                        class: "btn primary",
                        href: "{RESUME.resume_pdf}",
                        download: "Mohd_Arafat_Hossain_Resume.pdf",
                        "Download PDF"
                    }
                }
            }

            div { class: "resume-grid",
                aside { class: "resume-stack",
                    section { class: "resume-section",
                        h2 { "Summary" }
                        p { "{RESUME.summary}" }
                    }

                    section { class: "resume-section",
                        h2 { "Skills" }

                        div { class: "skill-groups",
                            for group in SKILL_GROUPS {
                                div { class: "skill-group",
                                    h3 { "{group.title}" }

                                    div { class: "pill-row",
                                        for skill in group.skills {
                                            span { class: "pill", "{skill}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "resume-stack",
                    section { class: "resume-section",
                        h2 { "Experience" }

                        div { class: "timeline",
                            for item in EXPERIENCE {
                                article { class: "timeline-item",
                                    h3 { "{item.role}" }
                                    p { class: "meta", "{item.company} • {item.period}" }
                                    p { "{item.details}" }

                                    ul { class: "resume-bullets",
                                        for bullet in item.bullets {
                                            li { "{bullet}" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    section { class: "resume-section",
                        h2 { "Projects" }

                        div { class: "timeline",
                            for project in PROJECTS {
                                article { class: "timeline-item",
                                    h3 { "{project.title}" }
                                    p { class: "meta", "{project.category}" }
                                    p { "{project.details}" }

                                    div { class: "pill-row",
                                        for tech in project.tech {
                                            span { class: "pill", "{tech}" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    section { class: "resume-section",
                        h2 { "Education" }

                        div { class: "timeline",
                            for item in EDUCATION {
                                article { class: "timeline-item",
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
}