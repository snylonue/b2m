#[macro_export]
macro_rules! matched {
    ($url:expr, $reg: expr) => {
        {
            let reg = regex::Regex::new($reg).unwrap();
            reg.is_match($url)
        }
    };
    ($url:expr, $($regs: expr),*) => {
        
        {
            let regs = regex::RegexSet::new(&[
                $($regs,)*
            ]).unwrap();
            regs.is_match($url)
        }
    };
}
#[macro_export]
macro_rules! value_to_string {
    ($v: expr) => {
        match $v {
            serde_json::Value::String(ref s) => Some(s.clone()),
            _ => None,
        }
    };
    ($v: expr, $or: expr) => {
        match $v {
            serde_json::Value::String(ref s) => Some(s.clone()),
            _ => $crate::value_to_string!($or),
        }
    };
}
#[macro_export]
macro_rules! parse_json {
    ($s: expr) => {
        match serde_json::from_str($s) {
            Ok(v) => v,
            Err(e) => return Err(failure::err_msg(format!("Failed to deserialize: {}", e))),
        }
    };
    ($s: expr, $err_msg: expr) => {
        match serde_json::from_str($s) {
            Ok(v) => v,
            Err(_) => return Err($err_msg),
        }
    };
}

pub mod bilibili;
pub mod iqiyi;

use serde_json::Value;
use std::process;
use super::Res;

type ResultInfo = super::Res<super::MediaInfo>;

pub struct Url {
    pub videos: Vec<String>,
    pub audios: Vec<String>,
}

pub trait Parser {
    fn is_support(url: &str)  -> bool;
    fn parse(url: &str) -> ResultInfo;
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
pub fn parse_annie_url(value: &Value) -> Option<Url> {
    match value["site"].as_str()? {
        "哔哩哔哩 bilibili.com" => {
            bilibili::parse_annie(value)
        }
        _ => None,
    }
}