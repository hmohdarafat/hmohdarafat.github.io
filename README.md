# hmohdarafat.github.io

Dioxus Rust/WASM website for:

- Homepage
- Resume page
- Portfolio page
- Blog index showing the latest 10 posts
- Individual blog post pages
- Contact section with `hmohdarafat@gmail.com`

## Run locally

```bash
cargo install dioxus-cli
cargo install wasm-bindgen-cli

dx serve --platform web
```

Open the local URL shown by `dx serve`.

## Build for GitHub Pages

This repo is intended for the user-site domain:

```text
https://hmohdarafat.github.io
```

Because this is a root user site, `Dioxus.toml` does not set `base_path`.

### Linux/macOS/Git Bash

```bash
bash scripts/build-pages.sh
```

### Windows PowerShell

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\build-pages.ps1
```

Then commit and push:

```bash
git add .
git commit -m "Build Dioxus portfolio site"
git push origin main
```

In GitHub repository settings, set GitHub Pages source to:

```text
Deploy from a branch
Branch: main
Folder: /docs
```

## Add a new blog post

1. Create a new file in `src/content/blogs/`, for example:

```text
src/content/blogs/my_new_post.rs
```

2. Add this inside the file:

```rust
use super::BlogPostData;

pub const POST: BlogPostData = BlogPostData {
    slug: "my-new-post",
    title: "My New Post",
    date: "2026-06-01",
    description: "Short summary shown on the blog index.",
    tags: &["Rust", "Dioxus"],
    body: r#"
Write the post here.

## Optional section

More text here.
"#,
};
```

3. Register it in `src/content/blogs/mod.rs`:

```rust
pub mod my_new_post;
```

4. Add it near the top of `POSTS` if it is newer:

```rust
my_new_post::POST,
```

The blog index automatically shows only the first 10 posts from `POSTS`, so keep that list sorted newest-first.
