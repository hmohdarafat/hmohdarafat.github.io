pub struct BlogPost {
    pub title: &'static str,
    pub date: &'static str,
    pub description: &'static str,
    pub tags: &'static [&'static str],
}

pub const BLOG_POSTS: &[BlogPost] = &[
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