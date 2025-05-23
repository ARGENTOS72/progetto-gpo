use crate::Route;
use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/styling/login.css");

#[component]
pub fn Login() -> Element {
    rsx! {
        link {
            rel: "stylesheet",
            href: CSS,
        }

        div { class: "container min-vh-100 d-flex justify-content-center align-items-center",
            
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
                                    Link {
                                        to: Route::Signup{},
                                        class: "nav-link",
                                        "Registrati"
                                    }
                                }
                            }
                        }
                    }
                
                
            }
        }
    }
}
