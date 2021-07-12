use super::Extractor;
use crate::parsers::Parser;
use crate::parsers::Url;
use crate::Config;
use serde_json::Value;

pub struct YouGet;
pub struct Annie;

impl Annie {
    const DISPLAYS: [&'static str; 27] = [
        "399", "398", "397", "396", "395", "394", "303", "302", "299", "298", "278", "251", "250",
        "249", "248", "247", "244", "243", "242", "160", "140", "137", "136", "135", "134", "133",
        "18",
    ];
}

impl Extractor for Annie {
    fn is_support(url: &str) -> bool {
        matched!(url, r"(https://)?(www\.)?youtube\.com/watch\?v=.")
    }
    fn real_url(value: &Value) -> Option<Url> {
        let (_, stream) = super::search_by_keys(&value["streams"], &Self::DISPLAYS)?;
        let urls = get!(&stream["parts"], &stream["urls"]);
        let video_url = value_to_string!(urls[0]["url"])?;
        let audio_url = value_to_string!(urls[1]["url"])?;
        Some(Url::new(vec![video_url], vec![audio_url]))
    }
    fn extract(url: &str, setting: &Config) -> crate::ResultInfo {
        crate::parsers::annie::Annie::parse(url, Self::real_url, setting)
    }
}
