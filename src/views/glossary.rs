use crate::{
    backend::glossary::get_chapters,
    components::{GlossaryContent, GlossaryNavigation},
};
use dioxus::prelude::*;
use log::debug;

use crate::backend::glossary::get_glossary_file_rsxed;

// const GLOSSARY_CSS: Asset = asset!("/assets/styling/[...]");
// fn convert_to_dioxus_links(raw_html: String) -> Vec<Element> {
//     let mut elements: Vec<Element> = Vec::new();

//     //regex (those who want to understand or know 💀). n.b.: 2 pz di regex
//     let re = Regex::new(r#"<a href="([^"]+)">([^<]+)</a>"#).unwrap();

//     let mut last_end = 0;//segna la fin

//     for cap in re.captures_iter(&raw_html) {//foreach match:
//         let href = &cap[1];
//         let link_text = &cap[2];

//         //rsx!a tutto prima del <a>
//         if last_end < cap.get(0).unwrap().start() {
//             let text_before = &raw_html[last_end..cap.get(0).unwrap().start()];
//             elements.push(rsx! { div { "{text_before}" } });
//         }

//         //fortnite building del link 👿
//         elements.push(rsx! {
//             Link { to: href, {link_text} }
//         });

//         //la fin della fin
//         last_end = cap.get(0).unwrap().end();
//     }

//     //rimane qualcosa della stringa? rsx!a
//     if last_end < raw_html.len() {
//         let remaining = &raw_html[last_end..];
//         elements.push(rsx! { div { "{remaining}" } });
//     }

//     elements
// }

#[component]
pub fn Glossary(chapter: String) -> Element {
    let chapters = use_server_future(move || async move { get_chapters().await })?;

    debug!("{}", chapter);

    let chapter = use_signal(|| chapter);

    rsx! {
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
