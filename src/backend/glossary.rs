use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs::File, io::BufReader, path::Path};
use std::io::{self, Read};  // Ensure `Read` is imported

use crate::error::Error;

const GLOSSARY_PATH: &str = "./glossary";

pub fn get_chapters() -> Result<Vec<String>, ServerFnError> {
    let reading_dir = std::fs::read_dir(GLOSSARY_PATH)?;

    let mut chapters = Vec::new();

    for file in reading_dir.into_iter().flatten() {
        let file_type = file.file_type().map_err(|e| {
            Error::Custom(format!(
                "Non si può leggere il tipo del file - {}",
                e.to_string()
            ))
        })?;

        if file_type.is_file() {
            chapters.push(
                file.file_name()
                    .to_str()
                    .ok_or(Error::Custom(String::from("Couldn't parse file name")))?
                    .split_once(".json")
                    .unwrap()
                    .0
                    .to_string(),
            );
        }
    }

    chapters.sort();

    Ok(chapters)
}

#[derive(Serialize, Deserialize)]
pub struct SubChapter {
    title: String,
    content: Vec<Content>,
}

#[derive(Serialize, Deserialize)]
struct Content {
    #[serde(flatten)]
    content_type: ContentType,
    text: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
enum ContentType {
    Paragraph,
    Code,
}

pub fn get_sub_chapters(chapter_name: &str) -> Result<Vec<SubChapter>, ServerFnError> {
    let file = File::open(
        Path::new(GLOSSARY_PATH)
            .join(chapter_name)
            .with_extension("json"),
    )?;

    let file_reader = BufReader::new(file);

    let subchapters: Vec<SubChapter> = {
        let value: Value = serde_json::from_reader(file_reader)?;

        serde_json::from_value(value["sub_chapters"].clone())?
    };

    Ok(subchapters)
}

//fin?
pub fn file_get_contents(file_name: &str) -> Result<String> {
    let file = File::open(Path::new(GLOSSARY_PATH).join(file_name).with_extension("html"))?;
    let mut contents = String::new();
    let mut reader = BufReader::new(file);
    
    reader.read_to_string(&mut contents)?;

    Ok(contents)
}
