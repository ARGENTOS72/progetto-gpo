use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/styling/login.css");

#[component]
pub fn Login() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }

        div { class: "container mt-5 d-flex justify-content-center align-items-center",
            div { class: "row w-100",
                div { class: "col-md-6 d-flex justify-content-center",
                    div { class: "form-container shadow p-4",
                        h3 { class: "mb-4", "LOG IN" }
                        form { id: "login-form",
                            div { class: "mb-3",
                                label { r#for: "email", class: "form-label", "Email" }
                                input {
                                    placeholder: "Email",
                                    r#type: "email",
                                    name: "email",
                                    class: "form-control",
                                    id: "email",
                                }
                            }
                            div { class: "mb-3",
                                label { r#for: "password", class: "form-label", "Password" }
                                input {
                                    placeholder: "Password",
                                    r#type: "password",
                                    name: "password",
                                    class: "form-control",
                                    id: "password",
                                }
                                a { href: "#", class: "text-secondary", "Password Dimenticata?" }
                            }
                            div { class: "d-flex justify-content-between align-items-center",
                                button {
                                    r#type: "submit",
                                    class: "btn btn-primary",
                                    "Login"
                                }
                            }
                            div { class: "mt-3",
                                p {
                                    "Non hai un account? "
                                    a { href: "#", class: "text-primary", "Registrati" }
                                }
                            }
                        }
                    }
                }
                div { class: "col-md-6 text-center d-flex flex-column align-items-center justify-content-center",
                    img {
                        src: "https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/1200px-Rust_programming_language_black_logo.svg.png",
                        alt: "Crab",
                        class: "crab mb-4",
                    }
                    h4 { "DIVENTA UN RUSTLER!" }
                    p { "PUNTI FONDAMENTALI:" }
                    ul { class: "text-start",
                        li { "Velocità" }
                        li { "Sicurezza" }
                        li { "Efficienza" }
                    }
                }
            }
        }
    }
}
