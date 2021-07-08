use super::search_by_keys;
use super::Extractor;
use crate::parsers::Parser;
use crate::parsers::Url;
use crate::Config;
use serde_json::Value;
use std::iter::FromIterator;

#[cfg(feature = "nfinata")]
mod fina {
    pub use finata::website::bilibili::Video;
    pub use finata::Config as _;
    pub use finata::ExtractSync;
    pub use netscape_cookie::parse;
    pub use std::fs::File;
    pub use std::io::Read;
}

#[cfg(feature = "nfinata")]
use fina::*;

pub struct YouGet;
pub struct Annie;
#[cfg(feature = "nfinata")]
pub struct Finata;

impl YouGet {
    const DISPLAYS: [&'static str; 8] = [
        "dash-flv",
        "flv",
        "dash-flv720",
        "flv720",
        "dash-flv480",
        "flv480",
        "dash-flv360",
        "flv360",
    ];
}
impl Extractor for YouGet {
    fn is_support(url: &str) -> bool {
        matched!(
            url,
            r"(?:https://)?(?:www\.)?bilibili\.com/(?:video/[AaBb][Vv]|bangumi/play/(?:ep|ss)).",
            r"(?:https://)?live\.bilibili\.com/\d"
        )
    }
    fn real_url(value: &Value) -> Option<Url> {
        //json['streams'] is ordered with BTreeMap
        let (dp, stream) = search_by_keys(&value["streams"], &Self::DISPLAYS)?;
        if dp.starts_with("dash") {
            let dash_url = &stream["src"];
            let video_url = vec![value_to_string!(dash_url[0][0])?];
            let audio_url = vec![value_to_string!(dash_url[1][0])?];
            Some(Url::new(video_url, audio_url))
        } else {
            let video_url = stream["src"]
                .as_array()?
                .iter()
                .filter_map(|x| value_to_string!(x))
                .collect();
            Some(Url::with_videos(video_url))
        }
    }
    #[inline]
    fn extract(url: &str, setting: &Config) -> crate::ResultInfo {
        crate::parsers::youget::YouGet::parse(url, Self::real_url, setting)
    }
}
impl Annie {
    const DISPLAYS: [&'static str; 9] = ["125", "120", "116", "112", "80", "74", "64", "32", "16"];
}
impl Extractor for Annie {
    fn is_support(url: &str) -> bool {
        matched!(
            url,
            r"(?:https://)?(?:www\.)?bilibili\.com/(?:(?:video/)?[AaBb][Vv]|bangumi/play/(?:ep|ss)).",
            r"(?:[AaBb][Vv]|ep)."
        )
    }
    fn real_url(value: &Value) -> Option<Url> {
        let (_, stream) = search_by_keys(&value["streams"], &Self::DISPLAYS)?;
        let urls = get!(&stream["parts"], &stream["urls"]);
        let videos = Vec::from_iter(value_to_string!(urls[0]["url"]));
        let audios = Vec::from_iter(value_to_string!(urls[1]["url"]));
        Some(Url::new(videos, audios))
    }
    #[inline]
    fn extract(url: &str, setting: &Config) -> crate::ResultInfo {
        crate::parsers::annie::Annie::parse(url, Self::real_url, setting)
    }
}
#[cfg(feature = "nfinata")]
impl Extractor for Finata {
    fn is_support(url: &str) -> bool {
        matched!(
            url,
            r"(?:https://)?(?:www\.)?bilibili\.com/(?:video/)?[AaBb][Vv]."
        )
    }
    fn real_url(_: &serde_json::Value) -> Option<Url> {
        None
    }
    fn extract(url: &str, conf: &Config) -> crate::ResultInfo {
        let mut extor = Video::new(url)?;
        if let Some(path) = conf.cookie {
            let mut cookie_file = File::open(path)?;
            let mut buf = Vec::new();
            cookie_file.read_to_end(&mut buf)?;
            let cookies: Vec<_> = parse(&buf)?
                .iter()
                .map(|cookie| format!("{}={}", cookie.name, cookie.value))
                .collect();
            extor.client_mut().push_cookie(&cookies.join(";"))?;
        }
        let info = extor.extract_sync()?;
        let (mut video, mut audio) = (vec![], vec![]);
        for track in &info.raws()[0].tracks {
            match track {
                finata::Track::Audio(url) => audio.push(url.to_string()),
                finata::Track::Video(url) => video.push(url.to_string()),
                _ => {}
            }
        }
        let url = Url::new(video, audio);
        Ok(crate::MediaInfo::new(
            url,
            Some(info.into_title()),
            Some("https://www.bilibili.com".to_owned()),
        ))
    }
}
