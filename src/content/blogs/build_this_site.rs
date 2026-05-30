use super::BlogPostData;

pub const POST: BlogPostData = BlogPostData {
    slug: "building-this-site-with-dioxus",
    title: "Building This Site with Dioxus",
    date: "2026-05-30",
    description: "Why this personal website is structured around Dioxus routes and content files.",
    tags: &["Dioxus", "Rust", "WASM"],
    body: r#"This site is built as a Dioxus web application. The main goal is to keep page routing separate from page content.

## Why Dioxus

Dioxus lets me write UI with Rust while still targeting the browser through WebAssembly. That makes it useful for learning stronger type-driven frontend patterns.

## Content organization

The resume content lives in one file. The portfolio content lives in one file. Each blog post lives in its own file.

## Benefit

When the website grows, I can add content without turning the main route file into a large mixed-content file.
"#,
};
