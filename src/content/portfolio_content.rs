pub struct Project {
    pub title: &'static str,
    pub category: &'static str,
    pub description: &'static str,
    pub tech: &'static [&'static str],
    pub link: &'static str,
}

pub const PROJECTS: &[Project] = &[
    Project {
        title: "hmohdarafat.github.io",
        category: "Personal website",
        description: "A Rust/Dioxus personal website combining resume, portfolio, and blog pages with a content-first folder structure.",
        tech: &["Rust", "Dioxus", "WASM", "GitHub Pages"],
        link: "https://github.com/hmohdarafat/hmohdarafat.github.io",
    },
    Project {
        title: "System Design Notes",
        category: "Interview preparation",
        description: "Structured notes and examples for distributed systems topics such as caching, quorum reads, LSM trees, Kafka, and backpressure.",
        tech: &["System Design", "Distributed Systems", "Markdown"],
        link: "https://github.com/hmohdarafat",
    },
    Project {
        title: "Rust Learning Projects",
        category: "Rust practice",
        description: "Small projects for learning ownership, modules, error handling, web apps, and practical Rust application structure.",
        tech: &["Rust", "CLI", "Linux"],
        link: "https://github.com/hmohdarafat",
    },
    Project {
        title: "Full-stack Admin Workflows",
        category: "Web application",
        description: "Admin-facing workflows with API integration, state handling, filtering, data export, and operational debugging.",
        tech: &["React", "Node.js", "MongoDB", "Redis"],
        link: "https://github.com/hmohdarafat",
    },
];
