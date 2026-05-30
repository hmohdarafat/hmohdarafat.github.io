use super::BlogPostData;

pub const POST: BlogPostData = BlogPostData {
    slug: "deploying-dioxus-to-github-pages",
    title: "Deploying Dioxus to GitHub Pages",
    date: "2026-05-28",
    description: "Notes on building a Dioxus site into the docs folder for GitHub Pages.",
    tags: &["GitHub Pages", "Deployment"],
    body: r#"GitHub Pages can host a static Dioxus web build.

## Deployment shape

For this user site, GitHub Pages should serve from the docs folder on the main branch.

## Routing note

Because the app uses client-side routing, the build script copies index.html to 404.html. That lets direct links such as /resume or /blog/post-slug still load the Dioxus app.

## Build step

The included scripts run dx bundle and prepare the docs folder for GitHub Pages.
"#,
};
