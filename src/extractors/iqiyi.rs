use serde_json::Value;
use super::search_by_keys;
use super::Extractor;
use crate::Setting;
use crate::parsers::Parser;
use crate::parsers::Url;

pub struct YouGet;
pub struct Annie;

impl YouGet {
    const DISPLAYS: [&'static str; 6] = ["TD_H265", "TD", "HD_H265", "HD", "SD", "LD"];
}
impl Extractor for YouGet {
    fn is_support(url: &str) -> bool {
        matched!(url,
            r"(?:https?://)?(?:www\.)?iqiyi\.com/."
        )
    }
    fn real_url(value: &Value) -> Option<Url> {
        let (_, stream) = super::search_by_keys(value, &Self::DISPLAYS)?;
        let video_url = stream["urls"]
            .as_array()?
            .iter()
            .filter_map(|x| value_to_string!(x["url"]))
            .collect();
        Some(Url::with_videos(video_url))
    }
    #[inline]
    fn extract(url: &str, setting: &Setting) -> crate::ResultInfo {
        crate::parsers::youget::YouGet::parse(url, Self::real_url, setting)
    }
}
impl Annie {
    const DISPLAYS: [&'static str; 4] = ["5", "4", "2", "1"];
}
impl Extractor for Annie {
    fn is_support(url: &str) -> bool {
        matched!(url,
            r"(?:https?://)?(?:www\.)?iqiyi\.com/."
        )
    }
    fn real_url(value: &Value) -> Option<Url> {
        let (_, stream) = search_by_keys(&value["streams"], &Self::DISPLAYS)?;
        let video_url = stream["src"]
            .as_array()?
            .iter()
            .filter_map(|x| value_to_string!(x))
            .collect();
        Some(Url::with_videos(video_url))
    }
    #[inline]
    fn extract(url: &str, setting: &Setting) -> crate::ResultInfo {
        crate::parsers::annie::Annie::parse(url, Self::real_url, setting)
    }
}