use super::Extractor;
use crate::parsers::Url;
use crate::Config;
use crate::MediaInfo;
use crate::ResultInfo;
use finata::website::netease_music::Song;
use finata::ExtractSync;
use netscape_cookie::parse;
use std::fs::File;
use std::io::Read;

pub struct Finata;

impl Extractor for Finata {
    fn is_support(url: &str) -> bool {
        matched!(url, r"(https?://)?music.163.com/#/song\?id=\d")
    }
    fn real_url(_: &serde_json::Value) -> Option<Url> {
        None
    }
    fn extract(url: &str, conf: &Config) -> ResultInfo {
        let mut song = Song::new(url.parse()?)?;
        if let Some(path) = conf.cookie {
            let mut cookie_file = File::open(path)?;
            let mut buf = Vec::new();
            cookie_file.read_to_end(&mut buf)?;
            let cookies: Vec<_> = parse(&buf)?
                .iter()
                .map(|cookie| format!("{}={}", cookie.name, cookie.value))
                .collect();
            song.client_mut().push_cookie(&cookies.join(";"))?;
        }
        let info = song.extract_sync()?;
        let url = Url::new(
            vec![],
            info.raws()[0]
                .tracks
                .iter()
                .map(|track| track.as_url().to_string())
                .collect(),
        );
        Ok(MediaInfo::new(url, Some(info.into_title()), None))
    }
}
