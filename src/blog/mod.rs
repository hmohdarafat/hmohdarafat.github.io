use dioxus::prelude::*;

mod data;

use data::BLOG_POSTS;

#[component]
pub fn Blog() -> Element {
    rsx! {
        section { id: "blog", class: "section",
            div { class: "section-heading",
                p { class: "eyebrow", "Blog" }
                h2 { "Writing" }
                p { "Short posts about Rust, web development, Linux, and system design." }
            }

            div { class: "grid cards",
                for post in BLOG_POSTS {
                    article { class: "card blog-card",
                        p { class: "date", "{post.date}" }
                        h3 { "{post.title}" }
                        p { "{post.description}" }
                        div { class: "tags",
                            for tag in post.tags {
                                span { "{tag}" }
                            }
                        }
                    }
                }
            }
        }
    }
}