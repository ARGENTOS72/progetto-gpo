use dioxus::prelude::*;

const CSS: Asset = asset!("/assets/styling/account.css");

#[component]
pub fn Account() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }

        div { class: "container min-vh-100 d-flex justify-content-center align-items-center",
            div { class: "row justify-content-center w-100",
                div { class: "col-12 col-md-10 col-lg-8",
                    div { class: "account-container shadow p-4 rounded-3 bg-white",
                        h3 { class: "mb-4 text-center border-bottom pb-3", "IL TUO ACCOUNT" }
                       
                        div { class: "row mb-5",
                            div { class: "col-md-6",
                                div { class: "account-info mb-4",
                                    h4 { class: "mb-3 text-secondary", "Informazioni Personalizzate" }
                                    ul { class: "list-group",
                                        li { class: "list-group-item d-flex justify-content-between",
                                            span { "Email registrata:" }
                                            span { class: "text-muted", "utente@esempio.com" }
                                        }
                                        li { class: "list-group-item d-flex justify-content-between",
                                            span { "Username:" }
                                            span { class: "text-muted", "rust_developer" }
                                        }
                                    }
                                }
                            }
                            
                            div { class: "col-md-6",
                                form { id: "account-form",
                                    h4 { class: "mb-3 text-secondary", "Modifica Dati" }

                                    div { class: "row g-3",
                                        div { class: "col-md-6",
                                            label { class: "form-label", "Nome" }
                                            input { 
                                                r#type: "text",
                                                class: "form-control",
                                                value: "Mario",
                                                required: true
                                            }
                                        }
                                        div { class: "col-md-6",
                                            label { class: "form-label", "Cognome" }
                                            input { 
                                                r#type: "text",
                                                class: "form-control",
                                                value: "Rossi",
                                                required: true
                                            }
                                        }
                                    }

                                    div { class: "mt-3",
                                        label { class: "form-label", "Numero di Telefono" }
                                        input { 
                                            r#type: "tel",
                                            class: "form-control",
                                            placeholder: "+39 123 456 7890",
                                            pattern: "[0-9]{10}"
                                        }
                                    }

                                    h4 { class: "mt-4 mb-3 text-secondary", "Sicurezza" }
                                    
                                    div { class: "row g-3",
                                        div { class: "col-md-6",
                                            label { class: "form-label", "Nuova Password" }
                                            input { 
                                                r#type: "password",
                                                class: "form-control",
                                                placeholder: "••••••••"
                                            }
                                        }
                                        div { class: "col-md-6",
                                            label { class: "form-label", "Conferma Password" }
                                            input { 
                                                r#type: "password",
                                                class: "form-control",
                                                placeholder: "••••••••"
                                            }
                                        }
                                    }

                                    div { class: "d-grid gap-3 mt-4",
                                        button { 
                                            r#type: "submit",
                                            class: "btn btn-primary w-100 py-2",
                                            "Aggiorna Account"
                                        }
                                        button { 
                                            r#type: "button",
                                            class: "btn btn-outline-danger w-100",
                                            "Elimina Account"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}