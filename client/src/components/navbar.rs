use crate::Route;
use dioxus::prelude::*;
use log::debug;

const CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    rsx! {

        link { rel: "stylesheet", href: CSS }

        nav { class: "navbar navbar-expand-sm bg-body-tertiary border-bottom border-body sticky-top",
            div { class: "container-fluid",
                div {
                    Link { to: Route::Home{},
                        img {
                            src: "https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg",
                            alt: "Rust",
                            width: "30",
                            class: "me-2",
                        }
                    }

                }

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
                    div {
                        class: "navbar-nav me-auto mb-2 mb-lg-0",
                        div {
                            class: "nav-item",
                            Link {
                                to: Route::Glossary{
                                    chapter: "00-00".to_string(),
                                },
                                class: "nav-link",
                                "Glossary"
                            }
                        }
                        div {class: "nav-item",
                            Link {
                                to: Route::Learning{},
                                class: "nav-link",
                                "Learning"
                            }
                        }
                        div {class: "nav-item",
                            Link {
                                to: Route::Playground{},
                                class: "nav-link",
                                "Playground"
                            }
                        }
                    }

                    div { class: "d-flex mx-2",
                        Link {
                            to: Route::Login{},
                            class: "btn btn-primary",
                            "Sign In"
                        }
                    }
                    div { class: "d-flex",
                        Link {
                            to: Route::Signup{},
                            class: "btn btn-secondary",
                            "Sign Up"
                        }
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
