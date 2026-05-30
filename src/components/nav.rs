use dioxus::prelude::*;

use crate::app::Route;

#[component]
pub fn Nav() -> Element {
    rsx! {
        header { class: "nav-wrapper",
            nav { class: "nav",
                Link {
                    class: "nav-brand",
                    to: Route::Home {},
                    "Mohd Arafat Hossain"
                }

                div { class: "nav-links",
                    Link {
                        to: Route::PortfolioPage {},
                        "Portfolio"
                    }

                    Link {
                        to: Route::ResumePage {},
                        "Resume"
                    }

                    Link {
                        to: Route::BlogPage {},
                        "Blog"
                    }
                }
            }
        }
    }
}