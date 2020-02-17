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
    pub videos: Option<Vec<String>>,
    pub audios: Option<Vec<String>>,
}
pub struct YouGet;
pub struct Annie;

pub trait Extractor {
    fn is_support(url: &str)  -> bool;
    fn extract(url: &str) -> ResultInfo;
}
pub trait Parser {
    //fn url(&self) -> &str;
    //fn extract(&self) -> Option<Url>;
    fn run(url: &str) -> Res<Value>;
    fn extract_infos(info: &Value) -> (Option<String>, Option<String>);
    fn parse<F>(url: &str, extractor: F) -> ResultInfo
        where F: Fn(&Value) -> Option<Url>
    {
        let infos = Self::run(url)?;
        let url = match extractor(&infos) {
            Some(url) => url,
            None => return Err(failure::err_msg("Failed to parse stdout as url")),
        };
        let (referrer, title) = Self::extract_infos(&infos);
        Ok(super::MediaInfo { url, referrer, title })
    }
}

impl Url {
    pub fn new(videos: Option<Vec<String>>, audios: Option<Vec<String>>) -> Self {
        Url { videos, audios }
    }
    pub fn with_all(videos: Vec<String>, audios: Vec<String>) -> Self {
        Self::new(Some(videos), Some(audios))
    }
}
impl Parser for YouGet {
    fn run(url: &str) -> Res<Value> {
        let (stdout, _) = super::cmd::run_command(process::Command::new("you-get")
            .arg(url)
            .arg("--json")
            .stderr(process::Stdio::null())
        )?;
        Ok(parse_json!(&stdout))
    }
    fn extract_infos(info: &Value) -> (Option<String>, Option<String>) {
        // referrer = json['extra']['referer'] || json_output['url']
        let referrer = value_to_string!(info["extra"]["referer"], info["url"]);
        let title = value_to_string!(info["title"]);
        (referrer, title)
    }
}
impl Parser for Annie {
    fn run(url: &str) -> Res<Value> {
        let (stdout, _) = super::cmd::run_command(process::Command::new("annie")
            .arg("-j")
            .arg(url)
            .stderr(process::Stdio::null())
        )?;
        Ok(parse_json!(&stdout))
    }
    fn extract_infos(info: &Value) -> (Option<String>, Option<String>) {
        let referrer = value_to_string!(info["url"]);
        let title = value_to_string!(info["title"]);
        (referrer, title)
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