use super::BlogPostData;

pub const POST: BlogPostData = BlogPostData {
    slug: "interview-prep-plan",
    title: "Interview Prep Plan",
    date: "2026-05-21",
    description: "How I organize learning for backend and system design interviews.",
    tags: &["Interview", "Learning"],
    body: r#"Interview preparation works better when each topic becomes a practiced answer.

## The pattern

For every topic, I prepare:

- Definition
- Example
- Tradeoffs
- Failure cases
- When to use it

## Goal

The goal is not to memorize every sentence. The goal is to explain clearly under pressure.
"#,
};
