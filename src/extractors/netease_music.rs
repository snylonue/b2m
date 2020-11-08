use super::Extractor;
use crate::parsers::Url;
use crate::Config;
use crate::MediaInfo;
use crate::ResultInfo;
use finata::website::netease_music::Song;
use tokio::runtime::Runtime;

pub use finata::website::netease_music::Song as Finata;

impl Extractor for Song {
    fn is_support(url: &str) -> bool {
        matched!(url, r"(?:https?://)?music.163.com/#/song\?id=\d")
    }

    fn real_url(_: &serde_json::Value) -> Option<Url> {
        None
    }

    fn extract(url: &str, _: &Config) -> ResultInfo {
        async fn _extract(url: &str) -> ResultInfo {
            let song = Song::new(url.parse()?);
            let info = song.extract().await?;
            let url = Url::new(
                vec![],
                info.raw_url
                    .into_iter()
                    .map(|(url, _)| url.to_string())
                    .collect(),
            );
            Ok(MediaInfo::with_ua(
                url,
                info.title,
                info.header
                    .get("referer")
                    .map(|v| v.to_str().unwrap().to_owned()),
                info.header
                    .get("user-agent")
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned(),
            ))
        }
        let mut runtime = Runtime::new().unwrap();
        runtime.block_on(_extract(url))
    }
}
