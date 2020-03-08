pub mod youget;
pub mod annie;

use serde_json::Value;
use super::Res;
use super::proxy;

type ResultInfo = super::Res<super::MediaInfo>;

pub struct Url {
    pub videos: Option<Vec<String>>,
    pub audios: Option<Vec<String>>,
}

pub trait Parser {
    fn run(url: &str, pxy: &Option<proxy::ProxyAddr>) -> Res<Value>;
    fn extract_infos(info: &Value) -> (Option<String>, Option<String>);
    fn parse<F>(url: &str, extractor: F, pxy: &Option<proxy::ProxyAddr>) -> ResultInfo
        where F: Fn(&Value) -> Option<Url>
    {
        let infos = Self::run(url, pxy)?;
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
    pub fn with_videos(videos: Vec<String>) -> Self {
        Self::new(Some(videos), None)
    }
}