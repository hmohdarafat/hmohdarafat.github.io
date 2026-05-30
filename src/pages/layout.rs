use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn SiteLayout() -> Element {
    rsx! {
        div { class: "site-shell",
            nav { class: "navbar",
                Link { to: Route::Home {}, class: "brand",
                    span { class: "brand-mark", "AH" }
                    span { "hmohdarafat.github.io" }
                }

                div { class: "nav-links",
                    Link { to: Route::Home {}, class: "nav-link", "Home" }
                    Link { to: Route::Resume {}, class: "nav-link", "Resume" }
                    Link { to: Route::Portfolio {}, class: "nav-link", "Portfolio" }
                    Link { to: Route::Blog {}, class: "nav-link", "Blog" }
                    a { class: "nav-link", href: "mailto:hmohdarafat@gmail.com", "Contact" }
                }
            }

            Outlet::<Route> {}

            footer { class: "footer",
                "© 2026 Mohd Arafat Hossain. Built with Rust, WebAssembly, and Dioxus."
            }
        }
    }
}
