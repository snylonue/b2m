use serde_json::Value;
use super::search_by_keys;
use super::Extractor;
use crate::Setting;
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
            r"(?:https://)?(?:www\.)?bilibili\.com/(?:video/[AaBb][Vv]|bangumi/play/ep).",
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
    fn extract(url: &str, setting: &Setting) -> crate::ResultInfo {
        crate::parsers::youget::YouGet::parse(url, Self::real_url, setting)
    }
}
impl Annie {
    const DISPLAYS: [&'static str; 8] = ["120", "116", "112", "80", "74", "64", "32", "16"];
}
impl Extractor for Annie {
    fn is_support(url: &str) -> bool {
        matched!(url, 
            r"(?:https://)?(?:www\.)?bilibili\.com/(?:video/[AaBb][Vv]|bangumi/play/ep).",
            r"(?:BV|ep|av)."
        )
    }
    fn real_url(value: &Value) -> Option<Url> {
        let (_, stream) = search_by_keys(&value["streams"], &Self::DISPLAYS)?;
        let urls = get!(&stream["parts"], &stream["urls"]);
        let videos = value_to_string!(urls[0]["url"]).into_iter().collect();
        let audios = value_to_string!(urls[1]["url"]).into_iter().collect();
        Some(Url::new(videos, audios))
    }
    #[inline]
    fn extract(url: &str, setting: &Setting) -> crate::ResultInfo {
        crate::parsers::annie::Annie::parse(url, Self::real_url, setting)
    }
}