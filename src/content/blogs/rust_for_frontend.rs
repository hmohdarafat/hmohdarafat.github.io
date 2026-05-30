use super::BlogPostData;

pub const POST: BlogPostData = BlogPostData {
    slug: "rust-for-frontend-development",
    title: "Rust for Frontend Development",
    date: "2026-05-27",
    description: "What changes when frontend UI is written with Rust instead of JavaScript.",
    tags: &["Rust", "Frontend"],
    body: r#"Rust changes the way frontend code feels. It pushes more mistakes into compile time.

## Strong types

Routes, props, and content structs can be checked before the site is deployed.

## Tradeoff

The learning curve is higher than a JavaScript-only stack, but the structure can become clearer once the project grows.
"#,
};
