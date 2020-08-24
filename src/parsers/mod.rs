pub mod youget;
pub mod annie;

use anyhow::Result;
use serde_json::Value;
use crate::ResultInfo;
use crate::MediaInfo;
use crate::Config;

/// A struct that contains two kinds of urls
#[derive(Debug)]
pub struct Url {
    pub videos: Vec<String>,
    pub audios: Vec<String>,
}

pub trait Parser {
    fn run(url: &str, setting: &Config) -> Result<Value>;
    /// Returns a tuple like (Some(referrer), Some(title))
    fn extract_infos(info: &Value) -> (Option<String>, Option<String>);
    fn parse<F>(url: &str, extractor: F, setting: &Config) -> ResultInfo
        where F: FnOnce(&Value) -> Option<Url>
    {
        let infos = Self::run(url, setting)?;
        let url = match extractor(&infos) {
            Some(url) => url,
            None => return Err(anyhow::anyhow!("No stream is found")),
        };
        let (referrer, title) = Self::extract_infos(&infos);
        Ok(MediaInfo::default_ua(url, title, referrer))
    }
}

impl Url {
    pub fn new(videos: Vec<String>, audios: Vec<String>) -> Self {
        Url { videos, audios }
    }
    pub fn with_videos(videos: Vec<String>) -> Self {
        Self::new(videos, Vec::new())
    }
}