use serde_json::Value;
use super::search_displays;
use super::Parser;
use super::Url;
use super::super::MediaInfo;

const YOU_GET_DISPLAYS: [&str; 6] = ["TD_H265", "TD", "HD_H265", "HD", "SD", "LD"];
const ANNIE_DISPLAYS: [&str; 4] = ["5", "4", "2", "1"];

pub struct YouGet;
pub struct Annie;

impl YouGet {
    fn real_url(value: &Value) -> Option<Url> {
        let (_, stream) = search_displays(&value["streams"], &YOU_GET_DISPLAYS)?;
        let video_url = stream["src"]
            .as_array()?
            .iter()
            .filter_map(|x| value_to_string!(x))
            .collect();
        Some(Url::new(video_url, Vec::new()))
    }
}
impl Parser for YouGet {
    fn is_support(url: &str) -> bool {
        matched!(url,
            r"(https?://)?(www.)?iqiyi.com/.+"
        )
    }
    fn parse(url: &str) -> super::ResultInfo {
        let infos = super::run_you_get(url)?;
        let url = match Self::real_url(&infos) {
            Some(url) => url,
            None => return Err(failure::err_msg("Failed to parse stdout as url")),
        };
        let (referrer, title) = super::you_get_infos(&infos);
        Ok(MediaInfo { url, referrer, title })
    }
}
impl Annie {
    fn real_url(value: &Value) -> Option<Url> {
        let (_, stream) = super::search_displays(value, &ANNIE_DISPLAYS)?;
        let video_url = stream["urls"]
            .as_array()?
            .iter()
            .filter_map(|x| value_to_string!(x["url"]))
            .collect();
        Some(Url::new(video_url, Vec::new()))
    }
}
impl Parser for Annie {
    fn is_support(url: &str) -> bool {
        matched!(url,
            r"(https?://)?(www.)?iqiyi.com/.+"
        )
    }
    fn parse(url: &str) -> super::ResultInfo {
        let infos = super::run_annie(url)?;
        let url = match Self::real_url(&infos["streams"]) {
            Some(url) => url,
            None => return Err(failure::err_msg("Failed to parse stdout as url")),
        };
        let (referrer, title) = super::annie_infos(&infos);
        Ok(MediaInfo { url, referrer, title })
    }
}