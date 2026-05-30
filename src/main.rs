#![allow(non_snake_case)]

use dioxus::prelude::*;

#[derive(Clone)]
struct Project {
    title: &'static str,
    description: &'static str,
    stack: &'static [&'static str],
    href: &'static str,
}

#[derive(Clone)]
struct BlogPost {
    title: &'static str,
    date: &'static str,
    description: &'static str,
    tags: &'static [&'static str],
}

const PROJECTS: &[Project] = &[
    Project {
        title: "Retina OMR Exam System",
        description: "A full-stack exam and OMR workflow system with admin/student interfaces, offline exam support, JWT authentication, MongoDB, Redis queues, and real-time features.",
        stack: &["React", "Node.js", "Express", "MongoDB", "Redis", "Socket.io"],
        href: "https://github.com/hmohdarafat",
    },
    Project {
        title: "Rust Portfolio Website",
        description: "This portfolio, resume, and blog website built with Dioxus and Rust WebAssembly, deployed on GitHub Pages.",
        stack: &["Rust", "Dioxus", "WASM", "GitHub Pages"],
        href: "https://github.com/hmohdarafat/hmohdarafat.github.io",
    },
    Project {
        title: "System Design Interview Notes",
        description: "Structured notes and examples for distributed systems topics such as caching, queues, replication, sharding, consistency, and backpressure.",
        stack: &["System Design", "Redis", "Kafka", "Databases", "Caching"],
        href: "https://github.com/hmohdarafat",
    },
];

const BLOG_POSTS: &[BlogPost] = &[
    BlogPost {
        title: "Why I Built My Portfolio with Rust and Dioxus",
        date: "2026-05-30",
        description: "A short explanation of using Rust, WebAssembly, Dioxus, and GitHub Pages for a personal developer website.",
        tags: &["Rust", "Dioxus", "WASM"],
    },
    BlogPost {
        title: "How I Study System Design for Interviews",
        date: "2026-05-30",
        description: "My approach to breaking down system design topics into active recall notes, examples, and interview-ready explanations.",
        tags: &["System Design", "Interview Prep"],
    },
    BlogPost {
        title: "Building Better Full-Stack Projects",
        date: "2026-05-30",
        description: "Notes on structuring full-stack applications with clear APIs, authentication, database models, caching, and deployment practices.",
        tags: &["Full Stack", "Architecture", "Backend"],
    },
];

const SKILLS: &[&str] = &[
    "Rust",
    "Dioxus",
    "JavaScript",
    "TypeScript",
    "React",
    "Redux Toolkit",
    "Node.js",
    "Express",
    "MongoDB",
    "Redis",
    "Docker",
    "Git",
    "Linux",
    "REST APIs",
    "System Design",
];

const EXPERIENCE_POINTS: &[&str] = &[
    "Built full-stack web applications using React, Node.js, Express, and MongoDB.",
    "Worked with authentication, role-based access control, REST APIs, and real-time features.",
    "Used Redis and background queues for caching and asynchronous processing.",
    "Designed data models, debugged production-style issues, and improved frontend state management.",
    "Currently learning Rust, Dioxus, Linux, and system design for stronger engineering depth.",
];

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Title { "Mohd Arafat Hossain | Portfolio" }
        document::Meta {
            name: "description",
            content: "Portfolio, resume, and blog of Mohd Arafat Hossain, full-stack developer."
        }
        document::Stylesheet { href: asset!("/assets/main.css") }

        div { class: "site",
            Nav {}
            main {
                Hero {}
                Portfolio {}
                Resume {}
                Blog {}
                Contact {}
            }
            Footer {}
        }
    }
}

#[component]
fn Nav() -> Element {
    rsx! {
        header { class: "nav-wrapper",
            nav { class: "nav",
                a { class: "brand", href: "#home", "Mohd Arafat Hossain" }
                div { class: "nav-links",
                    a { href: "#portfolio", "Portfolio" }
                    a { href: "#resume", "Resume" }
                    a { href: "#blog", "Blog" }
                    a { href: "#contact", "Contact" }
                }
            }
        }
    }
}

#[component]
fn Hero() -> Element {
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

#[component]
fn Portfolio() -> Element {
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

#[component]
fn Resume() -> Element {
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

#[component]
fn Blog() -> Element {
    rsx! {
        section { id: "blog", class: "section",
            div { class: "section-heading",
                p { class: "eyebrow", "Blog" }
                h2 { "Writing" }
                p { "Short posts about Rust, web development, Linux, and system design." }
            }

            div { class: "grid cards",
                for post in BLOG_POSTS {
                    article { class: "card blog-card",
                        p { class: "date", "{post.date}" }
                        h3 { "{post.title}" }
                        p { "{post.description}" }
                        div { class: "tags",
                            for tag in post.tags {
                                span { "{tag}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Contact() -> Element {
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

#[component]
fn Footer() -> Element {
    rsx! {
        footer { class: "footer",
            p { "© 2026 Mohd Arafat Hossain. Built with Rust, Dioxus, and GitHub Pages." }
        }
    }
}