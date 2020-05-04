#[macro_export]
macro_rules! value_to_string {
    ($v: expr) => {
        match $v {
            serde_json::Value::String(ref s) => Some(s.clone()),
            _ => None,
        }
    };
    ($v: expr, $or: expr) => {
        match $v {
            serde_json::Value::String(ref s) => Some(s.clone()),
            _ => $crate::value_to_string!($or),
        }
    };
}
macro_rules! try_get {
    ($s: expr) => {
        match $s {
            Ok(v) => v,
            Err(e) => return Err(e),
        }
    };
    ($s: expr, $err_msg: expr) => {
        match $s {
            Ok(v) => v,
            Err(_) => return Err(anyhow::anyhow!($err_msg)),
        }
    };
    ($s: expr; $err_msg: expr) => {
        match $s {
            Ok(v) => v,
            Err(e) => return Err(anyhow::anyhow!(format!($err_msg, e))),
        }
    };
}
#[macro_export]
macro_rules! parse_json {
    ($s: expr) => {
        try_get!(serde_json::from_str($s), format!("Invalid json data: {}", $s))
    };
}
macro_rules! find_parser {
    ($url: expr, $site: ident, $extractor: ident, $setting: expr) => {
        if $crate::extractors::$site::$extractor::is_support($url) {
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
        if let Some(urls) = videos {
            for i in urls {
                cmd.arg(i);
            }
            if let Some(aurls) = audios {
                for i in aurls {
                    cmd.arg(format!("--audio-file={}", i));
                }
            }
            if urls.len() > 1 {
                cmd.arg("--merge-files");
            }
        } else if let Some(urls) = audios {
            for i in urls {
                cmd.arg(i);
            }
            cmd.arg("--force-window=immediate");
        }
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
    #[cfg(feature= "annie")]find_parser!(url, bilibili, Annie, setting);
    #[cfg(feature= "youget")]find_parser!(url, bilibili, YouGet, setting);
    #[cfg(feature= "annie")]find_parser!(url, youtube, Annie, setting);
    #[cfg(feature= "annie")]find_parser!(url, iqiyi, Annie, setting);
    #[cfg(feature= "youget")]find_parser!(url, iqiyi, YouGet, setting);
    Err(anyhow::anyhow!("Unsupport url"))
}