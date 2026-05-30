use dioxus::prelude::*;

use crate::components::{Footer, Nav};
use crate::sections::{Contact, Hero};
use crate::portfolio::Portfolio;
use crate::resume::Resume;
use crate::blog::Blog;

#[component]
pub fn App() -> Element {
    rsx! {
        document::Title { "Mohd Arafat Hossain | Portfolio" }
        document::Meta {
            name: "description",
            content: "Portfolio, resume, and blog of Mohd Arafat Hossain, full-stack developer."
        }
        document::Stylesheet { href: asset!("/assets/main.css") }

        div { class: "site",
            Nav {}
            main {
                Hero {}
                Portfolio {}
                Resume {}
                Blog {}
                Contact {}
            }
            Footer {}
        }
    }
}