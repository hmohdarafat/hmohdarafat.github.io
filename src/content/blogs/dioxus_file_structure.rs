use super::BlogPostData;

pub const POST: BlogPostData = BlogPostData {
    slug: "dioxus-project-file-structure",
    title: "Dioxus Project File Structure",
    date: "2026-05-29",
    description: "A clean way to separate pages, content, and blog posts in a Rust web app.",
    tags: &["Architecture", "Dioxus"],
    body: r#"A personal website becomes hard to maintain when every page is written inside main.rs.

## The structure

- src/pages contains routed UI components
- src/content contains editable page content
- src/content/blogs contains one file per blog post

## Why it matters

This keeps the project easy to scan. Routes answer where a user can go. Content files answer what the user reads.
"#,
};
