use serde_json::Value;
use super::search_displays;
use super::Extractor;
use super::Parser;
use super::Url;

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
        Some(Url::new(Some(video_url), None))
    }
}
impl Extractor for YouGet {
    fn is_support(url: &str) -> bool {
        matched!(url,
            r"(https?://)?(www.)?iqiyi.com/."
        )
    }
    #[inline]
    fn extract(url: &str) -> super::ResultInfo {
        super::YouGet::parse(url, Self::real_url)
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
        Some(Url::new(Some(video_url), None))
    }
}
impl Extractor for Annie {
    fn is_support(url: &str) -> bool {
        matched!(url,
            r"(https?://)?(www.)?iqiyi.com/."
        )
    }
    #[inline]
    fn extract(url: &str) -> super::ResultInfo {
        super::Annie::parse(url, Self::real_url)
    }
}