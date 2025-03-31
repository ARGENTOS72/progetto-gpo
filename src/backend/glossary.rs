#![allow(unused)]

use dioxus::{dioxus_core::VText, prelude::*};
use ego_tree::NodeRef;
use log::debug;
use regex::Regex;
use scraper::{node::Element, Html, Node};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    io::{self, BufReader, Read},
    path::Path,
    sync::{Arc, Mutex},
};

use crate::{error::Error, Route};

const GLOSSARY_PATH: &str = "./glossary";

//#[server]
pub async fn get_chapters() -> Result<Vec<Chapter>, ServerFnError> {
    let reading_dir = std::fs::read_dir(GLOSSARY_PATH)?;

    let mut chapters: HashMap<String, Chapter> = HashMap::new();

    for file in reading_dir.into_iter().flatten() {
        let file_name = file.file_name().clone();
        let file_name = file_name.to_str().unwrap();

        // debug!("Leggendo ora {}", file_name);

        let re = Regex::new(
            r"^ch(?<first_digit>\d{2})-(?<second_digit>\d{2})-(?<title>[a-zA-Z0-9-]+)\.html$",
        )
        .unwrap();

        if re.is_match(file_name) {
            let Some(captures) = re.captures(file_name) else {
                continue;
            };

            let subchapter_unit = String::from(captures["second_digit"].to_string().clone());
            let chapter_unit = String::from(captures["first_digit"].to_string().clone());

            if subchapter_unit == "00" {
                let title = format!(
                    "{}. {}",
                    chapter_unit,
                    String::from(captures["title"].to_string().clone())
                );
                if let Some(mut ch_append) = chapters.get_mut(&chapter_unit) {
                    ch_append.content = read_to_string(file.path())?;
                    ch_append.title = title;
                    ch_append.id = format!("{}-{}", &chapter_unit, &subchapter_unit);
                } else {
                    chapters.insert(
                        chapter_unit.clone(),
                        Chapter::new(
                            title,
                            read_to_string(file.path())?,
                            format!("{}-{}", &chapter_unit, &subchapter_unit),
                            Vec::new(),
                        ),
                    );
                }
            } else {
                let title = format!(
                    "{}. {}",
                    subchapter_unit,
                    String::from(captures["title"].to_string().clone())
                );

                if (!chapters.contains_key(&chapter_unit)) {
                    chapters.insert(
                        chapter_unit.clone(),
                        Chapter::new(
                            format!("Capitolo {}", &chapter_unit),
                            String::from(""),
                            format!("{}-{}", &chapter_unit, &subchapter_unit),
                            Vec::new(),
                        ),
                    );
                }
                if let Some(mut ch_append) = chapters.get_mut(&chapter_unit) {
                    ch_append.sub_chapters.push(SubChapter::new(
                        title,
                        read_to_string(file.path())?,
                        format!("{}-{}", &chapter_unit, &subchapter_unit),
                    ));
                    ch_append.sub_chapters.sort();
                }
            }
        }
    }

    let mut ch_vec: Vec<Chapter> = chapters.into_values().collect();
    ch_vec.sort();

    Ok(ch_vec)
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chapter {
    title: String,
    content: String,
    id: String,
    sub_chapters: Vec<SubChapter>,
}

impl Chapter {
    fn new(title: String, content: String, id: String, sub_chapters: Vec<SubChapter>) -> Self {
        Self {
            title,
            content,
            sub_chapters,
            id,
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_sub_chapters(&self) -> Vec<SubChapter> {
        self.sub_chapters.clone()
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
    id: String,
}

impl SubChapter {
    pub fn new(title: String, content: String, id: String) -> Self {
        Self { title, content, id }
    }
    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_id(&self) -> &str {
        &self.id
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

pub fn get_glossary_body_rsxed(html: &mut String) -> Result<VNode, RenderError> {
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
        Some(Route::Glossary {
            chapter: ch,
        })
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
