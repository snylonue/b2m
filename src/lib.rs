#[macro_export]
macro_rules! value_to_string {
    ($v: expr) => {
        match $v {
            serde_json::Value::String(ref s) => Some(s.to_owned()),
            _ => None,
        }
    };
    ($v: expr, $($or: expr),+) => {
        match $v {
            serde_json::Value::String(ref s) => Some(s.to_owned()),
            _ => $crate::value_to_string!($($or),+),
        }
    };
}
#[macro_export]
macro_rules! parse_json {
    ($s: expr) => {
        match serde_json::from_str($s) {
            Ok(v) => v,
            Err(_) => return Err(anyhow::anyhow!(format!("Invalid json data: {}", $s))),
        }
    };
    ($s: expr, $ty: ty) => {
        match serde_json::from_str($s) {
            Ok::<$ty, _>(v) => v,
            Err(_) => return Err(anyhow::anyhow!(format!("Invalid json data: {}", $s))),
        }
    };
}
#[macro_export]
macro_rules! get {
    ($v: expr) => {
        $v
    };
    ($v: expr, $($vn: expr),+) => {
        match $v {
            serde_json::Value::Null => $crate::get!($($vn),+),
            _ => $v,
        }
    }
}
macro_rules! find_parser {
    ($url: expr, $site: ident, $extractor_name: expr, $extractor: ident, $setting: expr) => {
        if cfg!(feature = $extractor_name) && $crate::extractors::$site::$extractor::is_support($url) {
            return $crate::extractors::$site::$extractor::extract($url, $setting);
        }
    };
}

pub mod proxy;
pub mod command;
pub mod extractors;
pub mod parsers;

use anyhow::Result;
use std::process::Command;
use std::io::Result as IoResult;
use proxy::ProxyAddr;
use parsers::Url;
use extractors::Extractor;

pub(crate) type ResultInfo = Result<MediaInfo>;

#[derive(Debug)]
pub struct MediaInfo {
    pub url: Url,
    pub title: Option<String>,
    pub referrer: Option<String>,
    pub user_agent: Option<String>,
}
pub struct Setting<'a> {
    pub proxy_addr: Option<ProxyAddr<'a>>,
    pub cookie: Option<&'a str>,
}

impl MediaInfo {
    pub fn new(url: Url, title: Option<String>, referrer: Option<String>) -> Self {
        Self::with_ua(url, title, referrer, None)
    }
    pub fn with_ua(url: Url, title: Option<String>, referrer: Option<String>, user_agent: Option<String>) -> Self {
        Self { url, title, referrer, user_agent }
    }
    pub fn default_ua(url: Url, title: Option<String>, referrer: Option<String>) -> Self {
        Self::with_ua(url, title, referrer, Some(String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/75.0.3770.4 Safari/537.36")))
    }
    pub fn play(&self) -> IoResult<()> {
        self.as_command().output()?;
        Ok(())
    }
    /// Spwans commands to run mpv
    pub fn as_command(&self) -> Command {
        let Url { videos, audios } = &self.url;
        let mut cmd = Command::new("mpv");
        match videos.len() {
            0 => cmd.args(audios.iter())
                .arg("--force-window=immediate"),
            1 => cmd.arg(&videos[0])
                .args(audios
                    .iter()
                    .map(|a| format!("--audio-file={}", a))
                ),
            _ => cmd.args(videos.iter())
                .args(audios
                    .iter()
                    .map(|a| format!("--audio-file={}", a))
                )
                .arg("--merge-files"),
        };
        if let Some(referrer) = &self.referrer {
            cmd.arg(format!("--referrer={}", referrer));
        }
        if let Some(title) = &self.title {
            cmd.arg(format!("--title={}", title));
        }
        if let Some(user_agent) = &self.user_agent {
            cmd.arg(format!("--user-agent={}", user_agent));
        }
        cmd.arg("--no-ytdl");
        cmd
    }
}
impl<'a> From<ProxyAddr<'a>> for Setting<'a> {
    fn from(proxy_addr: ProxyAddr<'a>) -> Setting<'a> {
       Setting { proxy_addr: Some(proxy_addr), cookie: None }
    }
}
impl<'a> From<Option<ProxyAddr<'a>>> for Setting<'a> {
    fn from(proxy_addr: Option<ProxyAddr<'a>>) -> Setting<'a> {
       Setting { proxy_addr: proxy_addr, cookie: None }
    }
}
impl<'a> AsRef<Option<ProxyAddr<'a>>> for Setting<'a> {
    fn as_ref(&self) -> &Option<ProxyAddr<'a>> {
        &self.proxy_addr
    }
}

pub fn parse(url: &str, setting: &Setting) -> Result<MediaInfo> {
    find_parser!(url, bilibili, "annie", Annie, setting);
    find_parser!(url, bilibili, "youget", YouGet, setting);
    find_parser!(url, youtube, "annie", Annie, setting);
    find_parser!(url, iqiyi, "annie", Annie, setting);
    find_parser!(url, iqiyi, "youget", YouGet, setting);
    Err(anyhow::anyhow!("Unsupport url"))
}