#![allow(non_snake_case)]

use dioxus::prelude::*;

mod app;
mod components;
mod sections;
mod portfolio;
mod resume;
mod blog;

use app::App;

fn main() {
    launch(App);
}