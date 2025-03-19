#![allow(unused)]

use dioxus::dioxus_core::VText;
use dioxus::prelude::*;
use scraper::node::Element;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{self, Read};
use std::{fs::File, io::BufReader, path::Path};
use scraper::{Html, Node};
use ego_tree::NodeRef;
use crate::Route;

use crate::error::Error;

const GLOSSARY_PATH: &str = "./glossary";

/*
pub fn get_chapters() -> Result<Vec<String>, ServerFnError> {
    let reading_dir = std::fs::read_dir(GLOSSARY_PATH)?;

    let mut chapters = Vec::new();

    for file in reading_dir.into_iter().flatten() {
        let file_name = file.file_name().clone();
        let file_name = file_name.to_str().unwrap();

        let re =
            Regex::new(r"^ch(?<first_digit>\d{2})-(?<second_digit>\d{2})-[a-zA-Z0-9-]+\.html$")
                .unwrap();

        let mut index_load = 0;

        if re.is_match(file_name) {
            let (Some(captures), Some(load_chapter_capture)) =
                (re.captures(file_name), re.captures(&load_chapter))
            else {
                continue;
            };

            if &captures["second_digit"] != "00"
                && &captures["first_digit"] != &load_chapter_capture["first_digit"]
            {
                continue;
            }

            if file.path().is_file() && re.is_match(file_name) {
                if file.file_name().to_str().unwrap() == load_chapter {
                    let chapter = Chapter::new(
                        file_name.split_once(".html").unwrap().0.to_string(),
                        read_to_string(file.path())?,
                        Vec::new(),
                    );

                    chapters.push(chapter);

                    index_load = chapters.len() - 1;
                } else if &captures["first_digit"] == &load_chapter_capture["first_digit"] {
                    let chapter = chapters.get_mut(index_load).unwrap();

                    let sub_chapter = SubChapter::new(
                        file_name.split_once(".html").unwrap().0.to_string(),
                        read_to_string(file.path())?,
                    );

                    chapter.sub_chapters.push(sub_chapter);
                    chapter.sub_chapters.sort();
                } else {
                    let chapter = Chapter::new(
                        file_name.split_once(".html").unwrap().0.to_string(),
                        String::new(),
                        Vec::new(),
                    );

                    chapters.push(chapter);
                }
            }
        }
    }

    chapters.sort();

    Ok(chapters)
}
*/

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
    print!("1- function 'get_glossary_file_rsxed' start!\n");

    let file = File::open(
        Path::new(GLOSSARY_PATH)
            .join(file_name)
            .with_extension("html"),
    )?;

    let mut file_reader = BufReader::new(file);
    let mut html = String::new();
    
    file_reader.read_to_string(&mut html)?;

    print!("2- html readed!\n");
    print!("{html}");

    //Manipulate html with scarper
    let parsed_html = Html::parse_fragment(&html);
    let nodes = parsed_html.tree.root().children();
    
    print!("3- start converting html string to VNode!\n");

    let children_vnodes: Vec<VNode> = nodes.map(convert_node).collect();//all html file into Vec<VNode>

    //Wrap in a parent container
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
            // VNode::text(VText::new(text)) cazzo palle non funziona na minchia
            //N.B.: dovrebbe fare testo
            // VNode::text(text)
            //VNode::new(text_node).unwrap();
            return rsx!{" {text} "}.unwrap();
        }

        Node::Element(element_node) => {
            let tag_name = element_node.name.local.as_ref();

            //get all child
            let children_vnodes: Vec<VNode> = node.children().map(convert_node).collect();

            match tag_name {
                "a" => {
                    let raw_href = element_node.attr("href").unwrap_or("#");
                    
                    match to_route(raw_href) {
                        Some(route) => {
                            rsx! {
                                Link {
                                    to: route,
                                    style: "color: red;",
                                    {children_vnodes.into_iter()}
                                }
                            }.unwrap()
                        },
                        None => {
                            rsx! {
                                a {
                                    href: "{raw_href}",//need to be changed / handled
                                    style: "color: red;",
                                    {children_vnodes.into_iter()}
                                }
                            }.unwrap()
                        },
                    }
                    
                }

                //acceptable tag
                "p" | "h1" | "ul" | "li" | "div" | "strong" | "em" => {
                    rsx! {
                        {tag_name} {children_vnodes.into_iter()}
                    }.unwrap()
                }
                // "p" => rsx! { {tag_name} { children_vnodes } }.unwrap(),
                // "h1" => rsx! { h1 { children_vnodes } },
                // "ul" => rsx! { ul { children_vnodes } },
                // "li" => rsx! { li { children_vnodes } },
                // "div" => rsx! { div { children_vnodes } },
                // "strong" => rsx! { strong { children_vnodes } },
                // "em" => rsx! { em { children_vnodes } },

                //altra merda che ne so
                _ => rsx! { div { {children_vnodes.into_iter()} } }.unwrap(),
            }
        }

        _ => VNode::empty().unwrap(),//comments match: exclude them
        // _ => VNode::Text(VText::new(String::new())), //no nigga comments
    }
}

fn to_route(route: &str) -> Option<Route> {
    if route.starts_with("/glossary") {
        let ch : Option<String> = route.get(10..).map(String::from);//take out ONLY '/glossary#'
        Some(Route::Glossary {chapter: ch})
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
    // match route {
    //     "/" | "/home" => Some(Route::Home {}),
    //     "/glossary" => Some(Route::Glossary {}),
    //     "/login" => Some(Route::Login {} ),
    //     "/signup" => Some(Route::Signup {}),
    //     "/account" => Some(Route::Account {}),
    //     _ => None,
    // }
}

/*
fn convert_node<'a>(cx: Scope<'a>, node: &scraper::node::NodeRef) -> Element<'a> {
    match node.value() {
        Node::Text(text) => {
            cx.render(rsx! { "{text}" })
        }
        Node::Element(element) => {
            let tag = element.name();
            let children = node.children().map(|child| convert_node(cx, &child)).collect::<Vec<_>>();

            match tag {
                "a" => {
                    // Manually handle <a> tag: e.g., transform href or make it an internal navigation link.
                    let href = element.attr("href").unwrap_or("#");
                    cx.render(rsx! {
                        a {
                            href: "{href}", // You can modify this href as needed
                            style: "color: red;", // Example of adding custom style
                            children
                        }
                    })
                }
                "p" => cx.render(rsx! { p { children } }),
                "h1" => cx.render(rsx! { h1 { children } }),
                "ul" => cx.render(rsx! { ul { children } }),
                "li" => cx.render(rsx! { li { children } }),
                "div" => cx.render(rsx! { div { children } }),
                "strong" => cx.render(rsx! { strong { children } }),
                "em" => cx.render(rsx! { em { children } }),
                _ => {
                    // Default handler for any unknown tags
                    cx.render(rsx! { div { children } })
                }
            }
        }
        _ => cx.render(rsx! { "" }), // Ignore comments or other nodes
    }
}

*/

/*
//fin?
pub fn file_get_contents(file_name: &str) -> Result<String> {
    let file = File::open(Path::new(GLOSSARY_PATH).join(file_name).with_extension("html"))?;
    let mut contents = String::new();
    let mut reader = BufReader::new(file);

    reader.read_to_string(&mut contents)?;

    Ok(contents)
}
*/
