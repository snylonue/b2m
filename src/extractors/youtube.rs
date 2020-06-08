use serde_json::Value;
use super::Extractor;
use crate::parsers::Parser;
use crate::parsers::Url;
use crate::Setting;

pub struct YouGet;
pub struct Annie;

impl Annie {
    const DISPLAYS: [&'static str; 27] = ["399", "398", "397", "396", "395", "394", "303", "302", "299", "298", "278", "251", "250", "249", "248", "247", "244", "243", "242", "160", "140", "137", "136", "135", "134", "133", "18"];
}

impl Extractor for Annie {
    fn is_support(url: &str) -> bool {
        matched!(url,
            r"(?:https://)?(?:www\.)?youtube\.com/watch\?v=."
        )
    }
    fn real_url(value: &Value) -> Option<Url> {
        let (_, stream) = super::search_by_keys(&value["streams"], &Self::DISPLAYS)?;
        let video_url = value_to_string!(get!(&stream["parts"], &stream["urls"])["url"])?;
        let audio_url = value_to_string!(get!(&stream["parts"], &stream["urls"])["url"])?;
        Some(Url::with_all(vec![video_url], vec![audio_url]))
    }
    fn extract(url: &str, setting: &Setting) -> crate::ResultInfo {
        let mut res = crate::parsers::annie::Annie::parse(url, Self::real_url, setting);
        match res.iter_mut().next() {
            Some(media) => media.user_agent = Some(String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/72.0.3579.1 Safari/537.36")),
            None => {},
        };
        res
    }
}