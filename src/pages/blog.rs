use crate::content::blogs::{find_post, latest_posts};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Blog() -> Element {
    rsx! {
        main { class: "page-main",
            header { class: "page-header",
                p { class: "eyebrow", "Blog" }
                h1 { class: "page-title gradient-text", "Latest writing" }
                p { class: "page-lead", "The index below shows the latest 10 posts. Every post is stored in its own Rust file under src/content/blogs/." }
            }

            section { class: "post-grid",
                for post in latest_posts() {
                    Link { class: "post-card", to: Route::BlogPost { slug: post.slug.to_string() },
                        p { class: "meta", "{post.date}" }
                        h3 { "{post.title}" }
                        p { "{post.description}" }
                        div { class: "pill-row",
                            for tag in post.tags {
                                span { class: "pill", "{tag}" }
                            }
                        }
                        span { class: "read-more", "Read full post →" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn BlogPost(slug: String) -> Element {
    let post = find_post(&slug);

    rsx! {
        main { class: "page-main",
            match post {
                Some(post) => rsx! {
                    article { class: "article article-shell glass-card",
                        Link { to: Route::Blog {}, class: "btn", "← Back to blog" }
                        p { class: "eyebrow", "{post.date}" }
                        h1 { class: "gradient-text", "{post.title}" }
                        p { class: "page-lead", "{post.description}" }
                        div { class: "pill-row",
                            for tag in post.tags {
                                span { class: "pill", "{tag}" }
                            }
                        }
                        div { class: "article-body",
                            for block in post.body.split("\n\n") {
                                BlogTextBlock { block: block.to_string() }
                            }
                        }
                    }
                },
                None => rsx! {
                    div { class: "empty-state",
                        h1 { "Blog post not found" }
                        p { "No post exists for slug: {slug}" }
                        Link { to: Route::Blog {}, class: "btn primary", "Back to blog" }
                    }
                }
            }
        }
    }
}

#[component]
fn BlogTextBlock(block: String) -> Element {
    let trimmed = block.trim().to_string();

    if trimmed.is_empty() {
        return rsx! {};
    }

    if let Some(heading) = trimmed.strip_prefix("## ") {
        return rsx! { h2 { "{heading}" } };
    }

    if trimmed.starts_with("- ") {
        let items: Vec<String> = trimmed
            .lines()
            .filter_map(|line| line.trim().strip_prefix("- ").map(ToOwned::to_owned))
            .collect();

        return rsx! {
            ul {
                for item in items {
                    li { "{item}" }
                }
            }
        };
    }

    rsx! { p { "{trimmed}" } }
}
