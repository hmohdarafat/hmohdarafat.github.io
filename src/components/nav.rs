use dioxus::prelude::*;

#[component]
pub fn Nav() -> Element {
    rsx! {
        header { class: "nav-wrapper",
            nav { class: "nav",
                a { class: "brand", href: "#home", "Mohd Arafat Hossain" }
                div { class: "nav-links",
                    a { href: "#portfolio", "Portfolio" }
                    a { href: "#resume", "Resume" }
                    a { href: "#blog", "Blog" }
                    a { href: "#contact", "Contact" }
                }
            }
        }
    }
}