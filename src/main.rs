mod content;
mod pages;

use dioxus::prelude::*;
use pages::{Blog, BlogPost, Home, NotFound, Portfolio, Resume, SiteLayout};

static MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(SiteLayout)]
        #[route("/")]
        Home {},

        #[route("/resume")]
        Resume {},

        #[route("/portfolio")]
        Portfolio {},

        #[route("/blog")]
        Blog {},

        #[route("/blog/:slug")]
        BlogPost { slug: String },

        #[route("/:..route")]
        NotFound { route: Vec<String> },
}

#[component]
fn App() -> Element {
    rsx! {
        document::Title { "Mohd Arafat Hossain" }
        document::Stylesheet { href: MAIN_CSS }
        Router::<Route> {}
    }
}
