use finata::website::netease_cloud_music::Song;
use tokio::runtime::Runtime;
use super::Extractor;
use crate::Setting;
use crate::ResultInfo;
use crate::parsers::Url;
use crate::MediaInfo;

pub struct Finata;

impl Finata {
    pub async fn extract_async(url: &str) -> ResultInfo {
        let song = Song::new(url.parse()?);
        let info = song.extract().await?;
        let url = Url::new(
            vec![],
            info.raw_url
                .into_iter()
                .map(|(url, _)| url.to_string())
                .collect()
        );
        Ok(
            MediaInfo::with_ua(
                url,
                info.title,
                info.header.get("referer").map(|v| v.to_str().unwrap().to_owned()),
                info.header.get("user-agent").unwrap().to_str().unwrap().to_owned()
            )
        )
    }
}
impl Extractor for Finata {
    fn is_support(url: &str) -> bool {
        matched!(
            url,
            r"(?:https?://)?music.163.com/#/song\?id=\d"
        )
    }
    fn real_url(_: &serde_json::Value) -> Option<Url> {
        None
    }
    fn extract(url: &str, _: &Setting) -> ResultInfo {
        let mut runtime = Runtime::new().unwrap();
        runtime.block_on(Self::extract_async(url))
    }
}