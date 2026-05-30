use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn SiteLayout() -> Element {
    let current_route = use_route::<Route>();

    let home_active = matches!(&current_route, Route::Home {});
    let resume_active = matches!(&current_route, Route::Resume {});
    let portfolio_active = matches!(&current_route, Route::Portfolio {});
    let blog_active = matches!(&current_route, Route::Blog {});
    
    let navigator = use_navigator();

    rsx! {
        div { class: "site-shell",
            nav { class: "navbar",
                Link { to: Route::Home {}, class: "brand",
                    img {
                        class: "brand-image",
                        src: asset!("/assets/images/profile_pic_mini.jpg"),
                        alt: "Mohd Arafat Hossain profile picture"
                    }

                    span { class: "brand-text",
                        span { class: "brand-name", "Mohd Arafat Hossain" }
                        span { class: "brand-bracket", " [ " }
                        span { class: "brand-domain", "hmohdarafat.github.io" }
                        span { class: "brand-bracket", " ] " }
                    }
                }

                div { class: "nav-links",
                    Link {
                        to: Route::Home {},
                        class: if home_active { "nav-link active" } else { "nav-link" },
                        "Home"
                    }

                    Link {
                        to: Route::Resume {},
                        class: if resume_active { "nav-link active" } else { "nav-link" },
                        "Resume"
                    }

                    Link {
                        to: Route::Portfolio {},
                        class: if portfolio_active { "nav-link active" } else { "nav-link" },
                        "Portfolio"
                    }

                    Link {
                        to: Route::Blog {},
                        class: if blog_active { "nav-link active" } else { "nav-link" },
                        "Blog"
                    }

                    a {
                        class: "nav-link",
                        href: "/#contact",
                        onclick: move |event| {
                            event.prevent_default();

                            if home_active {
                                document::eval(
                                    r#"
                                    const contactSection = document.getElementById('contact');

                                    if (contactSection) {
                                        window.history.replaceState(null, '', '/#contact');

                                        contactSection.scrollIntoView({
                                            behavior: 'smooth',
                                            block: 'start'
                                        });
                                    }
                                    "#,
                                );
                            } else {
                                document::eval(
                                    r#"
                                    sessionStorage.setItem('scrollToContact', 'true');
                                    "#,
                                );

                                navigator.push(Route::Home {});
                            }
                        },
                        "Contact"
                    }
                }
            }

            Outlet::<Route> {}

            footer { class: "footer",
                "© 2026 Mohd Arafat Hossain. Built with Rust, WebAssembly, and Dioxus."
            }
        }
    }
}