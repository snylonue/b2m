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

pub mod proxy;
pub mod command;
pub mod extractors;
pub mod parsers;

use anyhow::Result;
use std::process::Command;
use std::io::Result as IoResult;
use proxy::ProxyAddr;
use parsers::Url;

type ResultInfo = Result<MediaInfo>;

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
        Self::with_ua(url, title, referrer, Some(String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/72.0.3579.1 Safari/537.36")))
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
                ),
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
impl<'a> Setting<'a> {
    pub fn new(proxy_addr: Option<ProxyAddr<'a>>, cookie: Option<&'a str>) -> Self {
        Self { proxy_addr, cookie }
    }
}