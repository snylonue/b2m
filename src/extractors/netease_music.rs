use super::Extractor;
use crate::parsers::Url;
use crate::Config;
use crate::MediaInfo;
use crate::ResultInfo;
use finata::website::netease_music::Song;
use finata::Extract;
use finata::Origin;
use netscape_cookie::parse;
use std::fs::File;
use std::io::Read;
use tokio::runtime::Runtime;

pub struct Finata;

impl Finata {
    pub async fn extract_async<'a>(url: &str, conf: &Config<'a>) -> ResultInfo {
        let mut song = Song::new(url.parse()?)?;
        match conf.cookie {
            Some(path) => {
                let mut cookie_file = File::open(path)?;
                let mut buf = Vec::new();
                cookie_file.read(&mut buf)?;
                let cookies = parse(&buf).unwrap();
                let client = song.client_mut();
                for cookie in cookies {
                    client.push_cookie(&format!("{}={}", cookie.name, cookie.value))?;
                }
            }
            None => {}
        }
        let info = Extract::extract(&mut song).await?;
        let url = Url::new(
            vec![],
            info.raws()
                .iter()
                .map(|&Origin { ref url, .. }| url.to_string())
                .collect(),
        );
        Ok(MediaInfo::new(url, Some(info.into_title()), None))
    }
}
impl Extractor for Finata {
    fn is_support(url: &str) -> bool {
        matched!(url, r"(?:https?://)?music.163.com/#/song\?id=\d")
    }
    fn real_url(_: &serde_json::Value) -> Option<Url> {
        None
    }
    fn extract(url: &str, conf: &Config) -> ResultInfo {
        let runtime = Runtime::new().unwrap();
        runtime.block_on(Self::extract_async(url, conf))
    }
}
