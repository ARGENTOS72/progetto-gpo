use dioxus::prelude::*;

use crate::components::Navbar;
use crate::views::{Account, Glossary, Home, Login, Signup};
use crate::backend::learning_level::{get_level};

mod backend;
mod components;
mod error;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},

    #[route("/glossary")]
    Glossary {
        chapter: Option<String>,
    },

    #[route("/login")]
    Login {},

    #[route("/signup")]
    Signup {},

    #[route("/account")]
    Account {},

    #[route("/learning")]
    Learning {}
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const MAIN_SCSS: Asset = asset!("/assets/styling/custom.scss");

fn main() {
    //get_level().unwrap();

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️
    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_SCSS }

        div {
            style: "height: 100%",
            Router::<Route> {}
        }
    }

}
