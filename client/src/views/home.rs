use dioxus::prelude::*;

use crate::Route;

const CSS: Asset = asset!("/assets/styling/home.css");

#[component]
pub fn Home() -> Element {
    rsx! {
        link {
            rel: "stylesheet",
            href: CSS,
        }

        section { class: "hero-section",
            div { class: "container",
                h1 { class: "display-4", "Benvenuto nell'app di apprendimento su rust" }
                p { class: "lead",
                    "Apprenderai le parti fondamentali di rust e diventerai un esperto rustiano."
                }
            }
        }
        div { class: "container my-5",
            div { class: "row",
                div { class: "col-md-4",
                    h3 { "Glossario" }
                    p {
                        "Il glossario è la sezione nel quale studierai o ripasserai le parti fondanti di rust, come le funzioni o le variabili."
                    }
                }
                div { class: "col-md-4",
                    h3 { "Playground" }
                    p {
                        "Una sezione nel quale tu puoi eseguire qualsiasi codice a tuo piacere giusto per provare, senza dover scaricare rust."
                    }
                }
                div { class: "col-md-4",
                    h3 { "Learning" }
                    p {
                        "Qui imparerai effetivamente a diventere un esperto e come iniziare ad usare rust."
                    }
                }
            }
        }
        div {  
            class:"justify-content-center d-flex align-items-end w-100 position-absolute bottom-0 start-50 translate-middle-x pb-1",
        
            div {
                class: "text-center text-break w-50 text-muted",
                h6 {
                    "Creato dal gruppo"
                }
                p {
                    "Tomas Mateo Maurino Maceira, Aron Capraro, Giacomo Pascualini, Gabriele Bardin, Yari Montanaro, Sebastiano Tiveron"
                } 
            }
        }

        // style { "scoped": "false", lang: "css",
        //     ".hero-section {\n  background-color: #f8f9fa;\n  padding: 50px 0;\n  text-align: center;\n}\nfooter {\n  background-color: #343a40;\n  color: white;\n  padding: 20px 0;\n  text-align: center;\n}"
        // }
    }
}
