pub mod bilibili;
pub mod iqiyi;

use serde_json::Value;

pub struct Url {
    pub videos: Vec<String>,
    pub audios: Vec<String>,
}

impl Url {
    pub fn new(videos: Vec<String>, audios: Vec<String>) -> Self {
        Url { videos, audios }
    }
}

pub fn search_displays<'a>(object: &'a Value, displays: &[&str]) -> Option<(&'a String, &'a Value)> {
    let object = object.as_object()?;
    let mut res = None;
    for i in displays.iter() {
        match object.iter().find(|(x, _)| { x == i }) {
            Some(el) => {
                res = Some(el);
                break;
            },
            None => continue,
        }
    }
    match res {
        Some(_) => res,
        None => Some(object.iter().next()?)
    }
}
pub fn parse_you_get_url(value: &Value) -> Option<Url> {
    match value["site"].as_str()? {
        "Bilibili" => {
            bilibili::parse_you_get(value)
        },
        "爱奇艺 (Iqiyi)" => {
            iqiyi::parse_you_get(value)
        },
        _ => None,
    }
}