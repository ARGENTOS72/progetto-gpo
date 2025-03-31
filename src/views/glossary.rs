use crate::{
    backend::glossary::get_chapters,
    components::{GlossaryContent, GlossaryNavigation},
};
use dioxus::prelude::*;
use log::debug;

#[component]
pub fn Glossary(chapter: String) -> Element {
    let chapters = use_server_future(move || async move { get_chapters().await })?;

    debug!("{}", chapter);

    let chapter = use_signal(|| chapter);

    rsx! {
        div {
            class: "d-flex",
            GlossaryNavigation {
                chapters: chapters.value()().unwrap().unwrap(), // Può dare problemi in caso commentare tutta la funzione
                current_subch: chapter,
            },
            GlossaryContent {
                chapters: chapters.value()().unwrap().unwrap(),
                chapter_id: chapter,
            }
        }
    }
}
