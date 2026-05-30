use super::BlogPostData;

pub const POST: BlogPostData = BlogPostData {
    slug: "how-i-study-system-design",
    title: "How I Study System Design",
    date: "2026-05-26",
    description: "A simple method for turning system-design reading into interview-ready understanding.",
    tags: &["System Design", "Interview"],
    body: r#"Reading system design passively is not enough. I need to turn each topic into a usable answer.

## My process

- Define the concept in plain language
- Draw the data flow
- List tradeoffs
- Connect it to an interview example

## Example

For caching, I do not only memorize Redis. I ask where cache sits, what it stores, how it expires, and what breaks when it is stale.
"#,
};
