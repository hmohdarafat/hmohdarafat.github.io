use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        main { class: "page-main",
            div { class: "empty-state",
                p { class: "eyebrow", "404" }
                h1 { class: "page-title", "Page not found" }
                p { "The route /{route.join(\"/\")} does not exist." }
                Link { to: Route::Home {}, class: "btn primary", "Go home" }
            }
        }
    }
}
