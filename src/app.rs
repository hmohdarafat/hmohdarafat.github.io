use dioxus::prelude::*;

use crate::blog::Blog;
use crate::components::{Footer, Nav};
use crate::portfolio::Portfolio;
use crate::resume::Resume;
use crate::sections::Hero;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},

    #[route("/portfolio")]
    PortfolioPage {},

    #[route("/resume")]
    ResumePage {},

    #[route("/blog")]
    BlogPage {},

    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Title { "Mohd Arafat Hossain | Portfolio" }
        document::Meta {
            name: "description",
            content: "Portfolio, resume, and blog of Mohd Arafat Hossain, full-stack developer."
        }
        document::Stylesheet { href: asset!("/assets/main.css") }

        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        document::Title { "Mohd Arafat Hossain | Home" }

        div { class: "site",
            Nav {}

            main {
                Hero {}

                section { class: "section home-links",
                    div { class: "container",
                        h2 { "Explore" }
                        p { class: "section-intro",
                            "Choose a section to view my work, resume, or writing."
                        }

                        div { class: "home-link-grid",
                            Link {
                                class: "home-link-card",
                                to: Route::PortfolioPage {},
                                h3 { "Portfolio" }
                                p { "View selected projects, technical work, and development experience." }
                            }

                            Link {
                                class: "home-link-card",
                                to: Route::ResumePage {},
                                h3 { "Resume" }
                                p { "See my skills, work experience, education, and professional profile." }
                            }

                            Link {
                                class: "home-link-card",
                                to: Route::BlogPage {},
                                h3 { "Blog" }
                                p { "Read notes, tutorials, and technical writing." }
                            }
                        }
                    }
                }
            }

            Footer {}
        }
    }
}

#[component]
fn PortfolioPage() -> Element {
    rsx! {
        document::Title { "Portfolio | Mohd Arafat Hossain" }

        div { class: "site",
            Nav {}

            main {
                Portfolio {}
            }

            Footer {}
        }
    }
}

#[component]
fn ResumePage() -> Element {
    rsx! {
        document::Title { "Resume | Mohd Arafat Hossain" }

        div { class: "site",
            Nav {}

            main {
                Resume {}
            }

            Footer {}
        }
    }
}

#[component]
fn BlogPage() -> Element {
    rsx! {
        document::Title { "Blog | Mohd Arafat Hossain" }

        div { class: "site",
            Nav {}

            main {
                Blog {}
            }

            Footer {}
        }
    }
}

#[component]
fn NotFound(route: Vec<String>) -> Element {
    let path = route.join("/");

    rsx! {
        document::Title { "Page Not Found | Mohd Arafat Hossain" }

        div { class: "site",
            Nav {}

            main {
                section { class: "section",
                    div { class: "container",
                        h1 { "Page not found" }
                        p {
                            "The page /{path} does not exist."
                        }
                        Link {
                            class: "button",
                            to: Route::Home {},
                            "Go back home"
                        }
                    }
                }
            }

            Footer {}
        }
    }
}