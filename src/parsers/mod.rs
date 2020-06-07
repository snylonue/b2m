pub mod youget;
pub mod annie;

use anyhow::Result;
use serde_json::Value;
use crate::ResultInfo;
use crate::MediaInfo;
use crate::Setting;

/// A struct that contains two kinds of urls
#[derive(Debug)]
pub struct Url {
    pub videos: Option<Vec<String>>,
    pub audios: Option<Vec<String>>,
}

pub trait Parser {
    fn run(url: &str, setting: &Setting) -> Result<Value>;
    /// Returns a tuple like (Some(referrer), Some(title))
    fn extract_infos(info: &Value) -> (Option<String>, Option<String>);
    fn parse<F>(url: &str, extractor: F, setting: &Setting) -> ResultInfo
        where F: FnOnce(&Value) -> Option<Url>
    {
        let infos = Self::run(url, setting)?;
        let url = match extractor(&infos) {
            Some(url) => url,
            None => return Err(anyhow::anyhow!("No stream is found")),
        };
        let (referrer, title) = Self::extract_infos(&infos);
        Ok(MediaInfo::new(url, title, referrer))
    }
}

impl Url {
    pub fn new(videos: Option<Vec<String>>, audios: Option<Vec<String>>) -> Self {
        Url { videos, audios }
    }
    pub fn with_all(videos: Vec<String>, audios: Vec<String>) -> Self {
        Self::new(Some(videos), Some(audios))
    }
    pub fn with_videos(videos: Vec<String>) -> Self {
        Self::new(Some(videos), None)
    }
}