use dioxus::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    fs::{read_to_string, File},
    io::BufReader,
    path::Path,
    sync::{Arc, Mutex},
};

use crate::error::Error;

const GLOSSARY_PATH: &str = "./glossary";

#[server]
pub async fn get_chapters_name() -> Result<Vec<Chapter>, ServerFnError> {
    let reading_dir = std::fs::read_dir(GLOSSARY_PATH)?;

    let mut chapters = Vec::new();

    for file in reading_dir.into_iter().flatten() {
        let file_name = file.file_name().clone();
        let file_name = file_name.to_str().unwrap();

        let re =
            Regex::new(r"^ch(?<first_digit>\d{2})-(?<second_digit>\d{2})-[a-zA-Z0-9-]+\.html$")
                .unwrap();

        if re.is_match(file_name) {
            let Some(captures) = re.captures(file_name) else {
                continue;
            };

            if &captures["first_digit"] == "00" {
                let chapter = Chapter::new(
                    file_name.split_once(".html").unwrap().0.to_string(),
                    read_to_string(file.path())?,
                    Vec::new(),
                );

                chapters.push(chapter);
            } else {
                let index = chapters
                    .iter()
                    .position(|chapter| &chapter.title == file_name.split_once(".html").unwrap().0)
                    .unwrap();

                let chapter = chapters.get_mut(index).unwrap();

                let sub_chapter = SubChapter::new(
                    file_name.split_once(".html").unwrap().0.to_string(),
                    read_to_string(file.path())?,
                );

                chapter.sub_chapters.push(sub_chapter);
                chapter.sub_chapters.sort();
            }
        }
    }

    chapters.sort();

    Ok(chapters)
}

// #[server]
// pub async fn load_sub_chapters(
//     mut chapters: Vec<Chapter>,
// ) -> Result<Vec<Chapter>, ServerFnError> {
//     let re = Regex::new(r"^ch(?<first_digit>\d{2})-(?<second_digit>\d{2})-[a-zA-Z0-9-]+\.html$")
//         .unwrap();

//     let reading_dir = std::fs::read_dir(GLOSSARY_PATH)?;

//     for file in reading_dir.into_iter().flatten() {
//         let file_name = file.file_name().clone();
//         let file_name = file_name.to_str().unwrap();

//         if re.is_match(&chapter_name) {
//             let (Some(captures), Some(file_name_captures)) =
//                 (re.captures(&chapter_name), re.captures(file_name))
//             else {
//                 return Err(Error::Custom("Couldn't match the file name".to_string()).into());
//             };

//             if captures["first_digit"] != file_name_captures["first_digit"] {
//                 continue;
//             }

//             let index = chapters
//                 .iter()
//                 .position(|chapter| chapter.get_title() == chapter_name)
//                 .unwrap();

//             let chapter = chapters.get_mut(index).unwrap();

//             if file_name_captures["second_digit"] == captures["first_digit"] {
//                 chapter.content = read_to_string(file.path())?;
//             } else {
//                 let sub_chapter = SubChapter::new(
//                     file_name.split_once(".html").unwrap().0.to_string(),
//                     read_to_string(file.path())?,
//                 );

//                 chapter.sub_chapters.push(sub_chapter);
//                 chapter.sub_chapters.sort();
//             }
//         }
//     }

//     Ok(chapters)
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chapter {
    title: String,
    content: String,
    sub_chapters: Vec<SubChapter>,
}

impl Chapter {
    fn new(title: String, content: String, sub_chapters: Vec<SubChapter>) -> Self {
        Self {
            title,
            content,
            sub_chapters,
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_sub_chapters(&self) -> &Vec<SubChapter> {
        &self.sub_chapters
    }
}

impl Ord for Chapter {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.title.cmp(&other.title)
    }
}

impl PartialOrd for Chapter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Chapter {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }
}

impl Eq for Chapter {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubChapter {
    title: String,
    content: String,
}

impl SubChapter {
    fn new(title: String, content: String) -> Self {
        Self { title, content }
    }
}

impl Ord for SubChapter {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.title.cmp(&other.title)
    }
}

impl PartialOrd for SubChapter {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SubChapter {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }
}

impl Eq for SubChapter {}

// pub fn get_chapters() -> Result<Vec<String>, ServerFnError> {
//     let reading_dir = std::fs::read_dir(GLOSSARY_PATH)?;

//     let mut chapters = Vec::new();

//     for file in reading_dir.into_iter().flatten() {
//         let file_type = file.file_type().map_err(|e| {
//             Error::Custom(format!(
//                 "Non si può leggere il tipo del file - {}",
//                 e.to_string()
//             ))
//         })?;

//         if file_type.is_file() {
//             chapters.push(
//                 file.file_name()
//                     .to_str()
//                     .ok_or(Error::Custom(String::from("Couldn't parse file name")))?
//                     .split_once(".json")
//                     .unwrap()
//                     .0
//                     .to_string(),
//             );
//         }
//     }

//     chapters.sort();

//     Ok(chapters)
// }

// #[derive(Serialize, Deserialize)]
// pub struct SubChapter {
//     title: String,
//     content: Vec<Content>,
// }

// #[derive(Serialize, Deserialize)]
// struct Content {
//     #[serde(flatten)]
//     content_type: ContentType,
//     text: String,
// }

// #[derive(Serialize, Deserialize)]
// #[serde(tag = "type")]
// #[serde(rename_all = "lowercase")]
// enum ContentType {
//     Paragraph,
//     Code,
// }

// pub fn get_sub_chapters(chapter_name: &str) -> Result<Vec<SubChapter>, ServerFnError> {
//     let file = File::open(
//         Path::new(GLOSSARY_PATH)
//             .join(chapter_name)
//             .with_extension("json"),
//     )?;

//     let file_reader = BufReader::new(file);

//     let subchapters: Vec<SubChapter> = {
//         let value: Value = serde_json::from_reader(file_reader)?;

//         serde_json::from_value(value["sub_chapters"].clone())?
//     };

//     Ok(subchapters)
// }

// //fin?
// pub fn file_get_contents(file_name: &str) -> Result<String> {
//     let file = File::open(Path::new(GLOSSARY_PATH).join(file_name).with_extension("html"))?;
//     let mut contents = String::new();
//     let mut reader = BufReader::new(file);

//     reader.read_to_string(&mut contents)?;

//     Ok(contents)
// }
//

// fn convert_node(node: NodeRef) -> VNode {
//     match node.value() {
//         Node::Text(text_node) => {
//             let text = text_node.text.to_string();
//             // VNode::text(VText::new(text)) cazzo palle non funziona na minchia
//             //N.B.: dovrebbe fare testo
//             VNode::text(text)

//         }

//         println!("NIFFA");

//         Node::Element(element_node) => {
//             let tag_name = element_node.name.local.as_ref();

//             //iteratore bambini rapiti
//             let children_vnodes: Vec<VNode> = node.children().map(convert_node).collect();

//             match tag_name {
//                 "a" => {
//                     let href = element_node.attr("href").unwrap_or("#");

//                     rsx! {
//                         a {
//                             href: "{href}",
//                             style: "color: red;",
//                             children_vnodes
//                         }
//                     }
//                 }

//                 //possibili tag
//                 "p" => rsx! { p { children_vnodes } },
//                 "h1" => rsx! { h1 { children_vnodes } },
//                 "ul" => rsx! { ul { children_vnodes } },
//                 "li" => rsx! { li { children_vnodes } },
//                 "div" => rsx! { div { children_vnodes } },
//                 "strong" => rsx! { strong { children_vnodes } },
//                 "em" => rsx! { em { children_vnodes } },

//                 //altra merda che ne so
//                 _ => rsx! { div { children_vnodes } },
//             }
//         }

//         _ => VNode::Text(VText::new(String::new())), //no nigga comments
//     }
// }
