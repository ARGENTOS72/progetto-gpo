// use std::collections::HashMap;

// use crate::{
//     backend::glossary::{get_chapters, get_sub_chapters, SubChapter},
//     components::GlossaryNavigation,
// };
// use dioxus::prelude::*;

// // const GLOSSARY_CSS: Asset = asset!("/assets/styling/[...]");

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
