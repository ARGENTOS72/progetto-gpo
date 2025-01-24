use crate::Route;
use dioxus::prelude::*;

// const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        // document::Link { rel: "stylesheet", href: NAVBAR_CSS }


        nav { class: "navbar navbar-expand-sm bg-body-tertiary border-bottom border-body sticky-top",
            div { class: "container-fluid",
                img {
                    src: "https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg",
                    alt: "Rust",
                    width: "30",
                    class: "me-2",
                }
                "\n        Ruster\n      "
                button {
                    "data-bs-toggle": "collapse",
                    "data-bs-target": "#navbarTogglerDemo01",
                    "aria-controls": "navbarTogglerDemo01",
                    r#type: "button",
                    "aria-expanded": "false",
                    "aria-label": "Toggle navigation",
                    class: "navbar-toggler",
                    span { class: "navbar-toggler-icon" }
                }
                div {
                    class: "collapse navbar-collapse",
                    id: "navbarTogglerDemo01",
                    ul { class: "navbar-nav me-auto mb-2 mb-lg-0",
                        li { class: "nav-item",
                            // router_link { to: "/", class: "nav-link", id: "home", "Home" }
                        }
                        li { class: "nav-item",
                            // router_link {
                            //     to: "/glossario",
                            //     class: "nav-link",
                            //     id: "glossario",
                            //     "Glossario"
                            // }
                        }
                        li { class: "nav-item",
                            a { class: "nav-link", id: "playground", "Playground" }
                        }
                    }
                    div { id: "account",
                        a { class: "btn btn-primary", id: "signin",
                            "\n            Registrati\n          "
                        }
                        // router_link {
                        //     to: "/accedi",
                        //     class: "btn btn-primary",
                        //     id: "login",
                        //     "\n            Accedi\n          "
                        // }
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
