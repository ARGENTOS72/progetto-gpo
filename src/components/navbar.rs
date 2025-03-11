use crate::Route;
use dioxus::{html::link, prelude::*};

// const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    rsx! {
            // document::Link { rel: "stylesheet", href: NAVBAR_CSS }


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
                            div {class: "nav-item",
                                a { class: "nav-link", id: "learning", "Learning" }
                            }
                            div {class: "nav-item",
    <<<<<<< HEAD
                                // Link {
                                //     to: Route::Glossary{},
                                //     class: "nav-link",
                                //     "Glossary"
                                // }
    =======
                                Link {
                                    to: Route::Login{},
                                    // to: Route::Glossary{},
                                    class: "nav-link",
                                    "Glossary"
                                }
    >>>>>>> 8d0066a1a6d29cc717995875f1de5adc5c607ed8
                            }
                            div {class: "nav-item",
                                a { class: "nav-link", id: "playground", "Playground" }
                            }
                        }
                        // ul { class: "navbar-nav me-auto mb-2 mb-lg-0",



                        // li { class: "nav-item",
                        // Link { to: Route::Glossary{
                        //     chapter: None,
                        // }, "Glossary" }
                        // ul { class: "navbar-nav me-auto mb-2 mb-lg-0",


                        // li { class: "nav-item",
                        //     a { class: "nav-link", id: "learning", "Learning" }
                        // }
                        // li { class: "nav-item",
                        //     Link {
                        //         to: Route::Glossary{},
                        //         class: "nav-link",
                        //         "Glossary"
                        //     }
                        // }
                        // li { class: "nav-item",
                        //     a { class: "nav-link", id: "playground", "Playground" }
                        // }
                        // }
                        div { class: "d-flex",
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
