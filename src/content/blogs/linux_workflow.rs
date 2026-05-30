use super::BlogPostData;

pub const POST: BlogPostData = BlogPostData {
    slug: "linux-workflow-for-developers",
    title: "Linux Workflow for Developers",
    date: "2026-05-22",
    description: "Small Linux habits that make development faster and easier to debug.",
    tags: &["Linux", "Workflow"],
    body: r#"A good Linux workflow is mostly about repeatable commands and readable project structure.

## Useful habits

- Learn ls, find, grep, awk, sed, and jq
- Keep scripts in a scripts folder
- Write commands down in README files

## Why it helps

When the same project needs to be rebuilt or deployed later, a documented command saves time and prevents mistakes.
"#,
};
