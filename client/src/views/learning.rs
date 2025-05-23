use dioxus::prelude::*;
use crate::backend::learning_level::{get_level};

const CSS: Asset = asset!("/assets/styling/home.css");

#[component]
pub fn Learning() -> Element {
    
    rsx! {
        
        h1 { class: "display-4", "Lavori in corso..." }
        p { class: "lead",
            "Ci scusiamo per il disagio"
        }
    }
}
