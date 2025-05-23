use dioxus::prelude::*;

use crate::backend::learning_level::*;
use crate::components::Navbar;
use crate::views::{Account, Glossary, Home, Learning, Login, Signup, Playground};

mod backend;
mod components;
mod error;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},

    #[route("/glossary/:chapter/")]
    Glossary { chapter: String },

    #[route("/login")]
    Login {},

    #[route("/signup")]
    Signup {},

    #[route("/account")]
    Account {},

    #[route("/learning")]
    Learning {},

    #[route("/playground")]
    Playground {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const MAIN_SCSS: Asset = asset!("/assets/styling/custom.scss");

fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::max())
        .filter_module("tokio_tungstenite", log::LevelFilter::Off)
        .filter_module("tungstenite", log::LevelFilter::Off)
        .filter_module("tao", log::LevelFilter::Off)
        .filter_module("html5ever", log::LevelFilter::Off)
        .filter_module("tracing", log::LevelFilter::Off)
        .init();
    // let lvl = get_level(1, 1).unwrap();
    // println!("Deserialized level from main: {:?}", lvl.clone());

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
