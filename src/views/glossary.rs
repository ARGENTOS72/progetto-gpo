use crate::{backend::glossary::get_chapters_name, components::GlossaryNavigation};
use dioxus::prelude::*;

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
pub fn Glossary(chapter: Option<String>) -> Element {
    let chapter = chapter.unwrap_or_else(|| "ch00-00-introduction.html".to_string());
    let cloned_chapter = chapter.clone();

    // let test = use_server_future(test);

    let chapters = use_server_future(move || {
        let chapter_cloned = cloned_chapter.clone();
        async move { get_chapters_name(chapter_cloned).await }
    })?;

    rsx! {
        GlossaryNavigation {
            chapters: chapters.value()().unwrap().unwrap(),
            current_chapter: chapter,
        }
        div {
            class: "p-4",
            style: "width: 75%; overflow: auto;",

            h1 {
                class: "mb-3",
            }
        }
    }
}
