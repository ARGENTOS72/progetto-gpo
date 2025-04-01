#![allow(unused)]

use dioxus::{dioxus_core::VText, prelude::*};
use ego_tree::NodeRef;
use log::debug;
use regex::Regex;
use scraper::{node::Element, ElementRef, Html, Node};
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
use std::env::current_dir;
use syntect::{highlighting::Highlighter, parsing::SyntaxSet};
use syntect::{
    highlighting::{Color, ThemeSet},
    html::highlighted_html_for_string,
};
use syntect::{html::highlighted_html_for_file, parsing::SyntaxReference};

use relative_path::RelativePath;

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
            let style = element_node.attr("style").unwrap_or_default();
            let class = element_node.attr("class").unwrap_or_default();

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
                "p" => rsx! { p { class: "{class}",style: "{style}",{children_vnodes.into_iter()} } }.unwrap(),
                "h1" => rsx! { h1 { class: "{class}",style: "{style}",{children_vnodes.into_iter()} } }.unwrap(),
                "ul" => rsx! { ul { class: "{class}",style: "{style}",{children_vnodes.into_iter()} } }.unwrap(),
                "ol" => rsx! { ol { class: "{class}",style: "{style}",{children_vnodes.into_iter()} } }.unwrap(),
                "li" => rsx! { li { class: "{class}",style: "{style}",{children_vnodes.into_iter()} } }.unwrap(),
                "div" => rsx! { div { class: "{class}",style: "{style}",{children_vnodes.into_iter()} } }.unwrap(),
                "strong" => rsx! { strong { {children_vnodes.into_iter()} } }.unwrap(),
                "pre" => rsx! { pre { class: "{class}",style: "{style}",{children_vnodes.into_iter()} } }.unwrap(),
                "em" => rsx! { em { class: "{class}",style: "{style}",{children_vnodes.into_iter()} } }.unwrap(),
                "code" => {
                    if( class.is_empty() ){
                        rsx! {
                            code {
                                class: "{class}",style: "{style}",
                                {children_vnodes.into_iter()}
                            }
                        }
                        .unwrap()
                    }else{
                        // let theme_content = Path::new(&"/home/redkitty/tr4nnysstuff/progetto-gpo/assets/styling/codetheme/")
                        let root = current_dir().unwrap_or_default();

                        // to_path unconditionally concatenates a relative path with its base:
                        let theme_content = RelativePath::new(&"../../assets/styling/codetheme/");
                        let full_path = theme_content.to_path(&root);

                        let mut theme_set = ThemeSet::load_defaults();
                        theme_set.add_from_folder(full_path).expect("Failed to load custom themes");
                        println!("Temi disponibili: {:?}", theme_set.themes.keys());
                        let ss = SyntaxSet::load_defaults_newlines();
                        let syntax = ss.find_syntax_by_extension("rs").unwrap();

                        let theme = &theme_set.themes["Erm"];
                        let c = theme.settings.background.unwrap_or(Color::BLACK);



                        let mut code_text = node
                            .children()
                            .filter_map(|child| match child.value() {
                                Node::Text(text) => Some(text.text.to_string()),
                                _ => None,
                            })
                            .collect::<Vec<String>>()
                            .join("");
                        // code_text= format!(
                        //     "<pre style=\"background-color: #272822; color: #f8f8f2; padding: 1em; border-radius: 5px;\">\n{}</pre>",
                        //     code_text
                        // );

                        let html = highlighted_html_for_string(&code_text, &ss, syntax, theme).unwrap();

                        let ercodice = Html::parse_fragment(&html);
                        let nodi_dercodice = ercodice.tree.root().children();

                        let i_bambini_dercodice: Vec<VNode> =
                            nodi_dercodice.map(convert_node).collect(); //all html file into Vec<VNode>
                        rsx! {
                            div {
                                class: "px-4 fs-6 py-2 justify-content-center codeblock",
                                code {
                                    class: "{class}, rounded",
                                    style: "{style}",
                                    {i_bambini_dercodice.into_iter()}
                                    // {html}
                                }
                            }

                        }
                        .unwrap()
                    }

                }
                "blockquote" => {
                    rsx! { blockquote {class: "{class}", style: "{style}",{children_vnodes.into_iter()} } }.unwrap()
                }
                "button" => {
                    rsx! { button { class: "{class}",style: "{style}",{children_vnodes.into_iter()} } }.unwrap()
                }
                "caption" => {
                    rsx! { caption {class: "{class}",style: "{style}", {children_vnodes.into_iter()} } }.unwrap()
                }
                "details" => {
                    rsx! { details {class: "{class}", style: "{style}",{children_vnodes.into_iter()} } }.unwrap()
                }
                "summary" => {
                    rsx! { summary { class: "{class}", style: "{style}",{children_vnodes.into_iter()} } }.unwrap()
                }
                "fieldset" => {
                    rsx! { fieldset {class: "{class}", style: "{style}", {children_vnodes.into_iter()} } }.unwrap()
                }
                "pre" => {
                    rsx! { pre {class: "{class}",style: "{style}", {children_vnodes.into_iter()} } }
                        .unwrap()
                }
                "br" => rsx! { br {} }.unwrap(),
                "hr" => rsx! { hr {} }.unwrap(),
                "kbd" => rsx! { kbd { {children_vnodes.into_iter()} } }.unwrap(),
                "span" => {
                    rsx! {
                        span {
                            class: "{class}",
                            style: "{style}",
                            {children_vnodes.into_iter()}
                        }
                    }
                    .unwrap()
                }
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
