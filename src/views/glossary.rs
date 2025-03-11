// use std::collections::HashMap;
// use dioxus::{html::g::dangerous_inner_html, prelude::*, Result};
// use crate::backend::glossary::file_get_contents;
// use crate::Route;
// use crate::{backend::glossary::{get_chapters, get_sub_chapters},
// components::GlossaryNavigation, views::Login};
// use regex::Regex;

// use crate::{
//     backend::glossary::{get_chapters, get_sub_chapters, SubChapter},
//     components::GlossaryNavigation,
// };
// use dioxus::prelude::*;

// // const GLOSSARY_CSS: Asset = asset!("/assets/styling/[...]");
// // fn convert_to_dioxus_links(raw_html: String) -> Vec<Element> {
// //     let mut elements: Vec<Element> = Vec::new();

// //     //regex (those who want to understand or know 💀). n.b.: 2 pz di regex
// //     let re = Regex::new(r#"<a href="([^"]+)">([^<]+)</a>"#).unwrap();

// //     let mut last_end = 0;//segna la fin

// //     for cap in re.captures_iter(&raw_html) {//foreach match:
// //         let href = &cap[1];
// //         let link_text = &cap[2];

// //         //rsx!a tutto prima del <a>
// //         if last_end < cap.get(0).unwrap().start() {
// //             let text_before = &raw_html[last_end..cap.get(0).unwrap().start()];
// //             elements.push(rsx! { div { "{text_before}" } });
// //         }

// //         //fortnite building del link 👿
// //         elements.push(rsx! {
// //             Link { to: href, {link_text} }
// //         });

// //         //la fin della fin
// //         last_end = cap.get(0).unwrap().end();
// //     }

// //     //rimane qualcosa della stringa? rsx!a
// //     if last_end < raw_html.len() {
// //         let remaining = &raw_html[last_end..];
// //         elements.push(rsx! { div { "{remaining}" } });
// //     }

// //     elements
// // }

// #[component]
// pub fn Glossary() -> Element {
//     // let mut readed_html = file_get_contents("test");
//     //readed_html è temp
//     let mut readed_html = String::from("<h2>Sono un H2!</h2><div><a href=\"/login\">login link</a></div>");
//     let readed_html_elements = convert_to_dioxus_links(readed_html);

//     rsx! {
//         GlossaryNavigation {}
//         div {
//             class: "p-4",
//             style: "width: 75%; overflow: auto;",

// #[derive(Debug, Serialize, Deserialize)]
// struct SubChapter {
//     title: String,
//     content: String,
// }

// #[component]
// pub fn Glossary(chapter: Option<String>) -> Element {
//     let chapters: HashMap<String, Vec<SubChapter>> = {
//         let chapters = get_chapters().unwrap();

//         let (chapter, sub_chapter) = chapter.map_or_else(
//             || {
//                 let chapter = chapters.first().unwrap().clone();

//                 (chapter, get_sub_chapters(&chapter).unwrap())
//             },
//             |chapter| (chapter, get_sub_chapters(&chapter).unwrap()),
//         );

//         chapters
//             .into_iter()
//             .map(|k_chapter| {
//                 if k_chapter == chapter {
//                     return (k_chapter, sub_chapter);
//                 }

//                 (k_chapter, Vec::new())
//             })
//             .collect()
//     };

//     rsx! {
//         // GlossaryNavigation {
//         //     chapters: chapters,
//         //     sub_chapters: sub_chapters,
//         // }
//         div {
//             class: "p-4",
//             style: "width: 75%; overflow: auto;",

//             h1 {
//                 class: "mb-3",
//                 // "{{ (subChaptersLoaded === undefined || subChaptersLoaded === null || Object.keys(subChaptersLoaded).length === 0) ? \"Loading...\" : subChaptersLoaded[chapterSelected.chapter][chapterSelected.subChapter] }}"
//             }
//         }
//     }
// }
// "{{ (subChaptersLoaded === undefined || subChaptersLoaded === null || Object.keys(subChaptersLoaded).length === 0) ? \"Loading...\" : subChaptersLoaded[chapterSelected.chapter][chapterSelected.subChapter] }}"
//             }

//             div {
//                 Link {
//                     to: Route::Login{},
//                     "Link prebuildato a login"
//                 }

//             }

//             div {
//                 // dangerous_inner_html: "{readed_html.as_ref().unwrap_or(&String::from(\"Errore...\"))}"

//                 //nn fnzia na bg
//                 {for readed_html_element in readed_html_elements.iter() {
//                     readed_html_element.clone();//master slave safe
//                 }}
//             }
//         }
//     }
// }
