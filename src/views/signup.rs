use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/styling/signup.css");

#[component]
pub fn Signup() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }

        div { class: "container min-vh-100 d-flex justify-content-center align-items-center",
            div { class: "row justify-content-center w-100",
                div { class: "col-12 col-md-8 col-lg-6",
                    div { class: "form-container shadow p-4 rounded-2",
                        h3 { class: "mb-4 text-center", "REGISTRAZIONE" }
                        form { id: "signup-form",
                            div { class: "mb-3",
                                label { r#for: "email", class: "form-label", "Email" }
                                input {
                                    placeholder: "esempio@email.com",
                                    r#type: "email",
                                    name: "email",
                                    class: "form-control",
                                    id: "email",
                                    required: true
                                }
                            }
                            
                            div { class: "mb-3",
                                div { class: "row g-3",
                                    div { class: "col-md-6",
                                        label { r#for: "password", class: "form-label", "Password" }
                                        input {
                                            placeholder: "Inserisci password",
                                            r#type: "password",
                                            name: "password",
                                            class: "form-control",
                                            id: "password",
                                            required: true
                                        }
                                    }
                                    div { class: "col-md-6",
                                        label { r#for: "password2", class: "form-label", "Conferma Password" }
                                        input {
                                            placeholder: "Ripeti password",
                                            r#type: "password",
                                            name: "password2",
                                            class: "form-control",
                                            id: "password2",
                                            required: true
                                        }
                                    }
                                }
                            }

                            div { class: "d-grid",
                                button {
                                    r#type: "submit",
                                    class: "btn btn-primary w-100 py-2",
                                    "Registrati ora"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}