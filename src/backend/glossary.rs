#![allow(unused)]

use dioxus::{dioxus_core::VText, prelude::*};
use ego_tree::NodeRef;
use regex::Regex;
use scraper::{node::Element, Html, Node};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    fs::{read_to_string, File},
    io::{self, BufReader, Read},
    path::Path,
    sync::{Arc, Mutex},
};
use log::debug;

use crate::{error::Error, Route};

const GLOSSARY_PATH: &str = "./glossary";

#[server]
pub async fn get_chapters() -> Result<Vec<Chapter>, ServerFnError> {
    let reading_dir = std::fs::read_dir(GLOSSARY_PATH)?;

    let mut chapters = Vec::new();

    for file in reading_dir.into_iter().flatten() {
        let file_name = file.file_name().clone();
        let file_name = file_name.to_str().unwrap();

        debug!("Leggendo ora {}", file_name);

        let re =
            Regex::new(r"^ch(?<first_digit>\d{2})-(?<second_digit>\d{2})-[a-zA-Z0-9-]+\.html$")
                .unwrap();

        if re.is_match(file_name) {
            let Some(captures) = re.captures(file_name) else {
                continue;
            };

            if &captures["second_digit"] == "00" {
                let chapter = Chapter::new(
                    file_name.split_once(".html").unwrap().0.to_string(),
                    read_to_string(file.path())?,
                    Vec::new(),
                );

                chapters.push(chapter);
            } else {
                let index = chapters.iter().position(|chapter| {
                    if let Some(captures_chapter) =
                        re.captures(&(chapter.get_title().to_string() + ".html"))
                    {
                        captures_chapter["first_digit"] == captures["first_digit"]
                    } else {
                        false
                    }
                });

                if let Some(index) = index {
                    let chapter = chapters.get_mut(index).unwrap();

                    let sub_chapter = SubChapter::new(
                        file_name.split_once(".html").unwrap().0.to_string(),
                        read_to_string(file.path())?,
                    );

                    chapter.sub_chapters.push(sub_chapter);
                    chapter.sub_chapters.sort();
                };
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

// pub enum GNode {
//     Element { tag: String, children: Vec<GNode> },
//     Text(String),
// }

// pub fn get_glossary_file_rsxed(file_name: &str) -> Result<VNode, RenderError> {
//     print!("1- function 'get_glossary_file_rsxed' start!\n");

//     let file = File::open(
//         Path::new(GLOSSARY_PATH)
//             .join(file_name)
//             .with_extension("html"),
//     )?;

//     let file_reader = BufReader::new(file);

//     let subchapters: Vec<SubChapter> = {
//         let value: Value = serde_json::from_reader(file_reader)?;

//         serde_json::from_value(value["sub_chapters"].clone())?
//     };

//     Ok(subchapters)
// }

// pub enum GNode {
//     Element { tag: String, children: Vec<GNode> },
//     Text(String),
// }

pub fn get_glossary_file_rsxed(file_name: &str) -> Result<VNode, RenderError> {
    let file = File::open(
        Path::new(GLOSSARY_PATH)
            .join(file_name)
            .with_extension("html"),
    )?;

    let mut file_reader = BufReader::new(file);
    let mut html = String::new();

    file_reader.read_to_string(&mut html)?;

    //Manipulate html with scarper
    let parsed_html = Html::parse_fragment(&html);
    let nodes = parsed_html.tree.root().children();

    let children_vnodes: Vec<VNode> = nodes.map(convert_node).collect(); //all html file into Vec<VNode>

    //wrap in a parent container
    rsx! {
        div {
            {children_vnodes.into_iter()}
        }
    }
}

fn convert_node(node: NodeRef<'_, Node>) -> VNode {
    match node.value() {
        Node::Text(text_node) => {
            let text = text_node.text.to_string();

            return rsx! {" {text} "}.unwrap();
        }

        Node::Element(element_node) => {
            let tag_name = element_node.name.local.as_ref();

            //get all child
            let children_vnodes: Vec<VNode> = node.children().map(convert_node).collect();

            match tag_name {
                "a" => {
                    let raw_href = element_node.attr("href").unwrap_or("#");

                    match to_route(raw_href) {
                        Some(route) => rsx! {
                            Link {
                                to: route,
                                style: "color: red;",
                                {children_vnodes.into_iter()}
                            }
                        }
                        .unwrap(),
                        None => rsx! {
                            a {
                                href: "{raw_href}",
                                style: "color: red;",
                                {children_vnodes.into_iter()}
                            }
                        }
                        .unwrap(),
                    }
                }

                //acceptable tag
                "p" => rsx! { p { {children_vnodes.into_iter()} } }.unwrap(),
                "h1" => rsx! { h1 { {children_vnodes.into_iter()} } }.unwrap(),
                "ul" => rsx! { ul { {children_vnodes.into_iter()} } }.unwrap(),
                "ol" => rsx! { ol { {children_vnodes.into_iter()} } }.unwrap(),
                "li" => rsx! { li { {children_vnodes.into_iter()} } }.unwrap(),
                "div" => rsx! { div { {children_vnodes.into_iter()} } }.unwrap(),
                "strong" => rsx! { strong { {children_vnodes.into_iter()} } }.unwrap(),
                "em" => rsx! { em { {children_vnodes.into_iter()} } }.unwrap(),
                "code" => rsx! { code { {children_vnodes.into_iter()} } }.unwrap(),
                "blockquote" => rsx! { blockquote { {children_vnodes.into_iter()} } }.unwrap(),
                "button" => rsx! { button { {children_vnodes.into_iter()} } }.unwrap(),
                "caption" => rsx! { caption { {children_vnodes.into_iter()} } }.unwrap(),
                "details" => rsx! { details { {children_vnodes.into_iter()} } }.unwrap(),
                "summary" => rsx! { summary { {children_vnodes.into_iter()} } }.unwrap(),
                "fieldset" => rsx! { fieldset { {children_vnodes.into_iter()} } }.unwrap(),
                "br" => rsx! { br {} }.unwrap(),
                "hr" => rsx! { hr {} }.unwrap(),

                //other: default as div
                _ => rsx! { div { {children_vnodes.into_iter()} } }.unwrap(),
            }
        }

        _ => VNode::empty().unwrap(), //comments match: exclude them
    }
}

fn to_route(route: &str) -> Option<Route> {
    if route.starts_with("/glossary") {
        let ch = route.get(10..).map(String::from).unwrap(); //take out ONLY '/glossary#'
        Some(Route::Glossary { chapter: ch })
    } else if route.starts_with("/login") {
        Some(Route::Login {})
    } else if route.starts_with("/signup") {
        Some(Route::Signup {})
    } else if route.starts_with("/account") {
        Some(Route::Account {})
    } else if route.starts_with("/home") || route == "/" {
        Some(Route::Home {})
    } else {
        None
    }
}
