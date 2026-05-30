pub struct Project {
    pub title: &'static str,
    pub description: &'static str,
    pub stack: &'static [&'static str],
    pub href: &'static str,
}

pub const PROJECTS: &[Project] = &[
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