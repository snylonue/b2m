use serde_json::Value;
use regex;
use super::search_displays;
use super::Extractor;
use crate::parsers::Parser;
use crate::parsers::Url;

pub struct YouGet;
pub struct Annie;

impl YouGet {
    const DISPLAYS: [&'static str; 8] = ["dash-flv", "flv", "dash-flv720", "flv720", "dash-flv480", "flv480", "dash-flv360", "flv360"];
}
impl Extractor for YouGet {
    fn is_support(url: &str) -> bool {
        matched!(url, 
            r"(?:https?://)?(?:www.)?bilibili.com/(?:video/av|bangumi/play/(?:ep|ss))\d",
            r"(?:https?://)?live.bilibili.com/\d"
        )
    }
    fn real_url(value: &Value) -> Option<Url> {
        //json['streams'] is ordered with BTreeMap
        let (dp, stream) = search_displays(&value["streams"], &Self::DISPLAYS)?;
        if dp.matches("dash").next().is_some() {
            let dash_url = stream["src"].as_array()?;
            let video_url = vec![value_to_string!(dash_url[0][0])?];
            let audio_url = vec![value_to_string!(dash_url[1][0])?];
            Some(Url::with_all(video_url, audio_url))
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
    fn extract(url: &str) -> super::ResultInfo {
        crate::parsers::youget::YouGet::parse(url, Self::real_url)
    }
}
impl Annie {
    const DISPLAYS: [&'static str; 4] = ["80", "64", "32", "16"];
}
impl Extractor for Annie {
    fn is_support(url: &str) -> bool {
        matched!(url, 
            r"(?:https?://)?(?:www.)?bilibili.com/(?:video/av|bangumi/play/ep)\d"
        )
    }
    fn real_url(value: &Value) -> Option<Url> {
        let (_, stream) = search_displays(&value["streams"], &Self::DISPLAYS)?;
        let urls = stream["urls"]
            .as_array()?
            .iter()
            .filter_map(|x| value_to_string!(x["url"]))
            .collect();
        Some(Url::with_videos(urls))
    }
    #[inline]
    fn extract(url: &str) -> super::ResultInfo {
        crate::parsers::annie::Annie::parse(url, Self::real_url)
    }
}