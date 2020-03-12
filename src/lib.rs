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
macro_rules! try_do {
    ($s: expr) => {
        match $s {
            Ok(v) => v,
            Err(e) => return Err(failure::err_msg(e.to_string())),
        }
    };
    ($s: expr, $err_msg: expr) => {
        match $s {
            Ok(v) => v,
            Err(_) => return Err(failure::err_msg($err_msg)),
        }
    };
    ($s: expr; $err_msg: expr) => {
        match $s {
            Ok(v) => v,
            Err(e) => return Err(failure::err_msg(format!($err_msg, e))),
        }
    };
}
#[macro_export]
macro_rules! parse_json {
    ($s: expr) => {
        try_do!(serde_json::from_str($s), format!("Failed to deserialize json data: {}", $s))
    };
}
macro_rules! find_parser {
    ($url: expr, $site: ident, $extractor: ident, $proxy: expr) => {
        if $crate::extractors::$site::$extractor::is_support($url) {
            return $crate::extractors::$site::$extractor::extract($url, $proxy);
        }
    };
}

pub mod proxy;
pub mod command;
pub mod extractors;
pub mod parsers;

use failure::err_msg;
use std::process;
use parsers::Url;
use extractors::Extractor;

pub type Res<T> = Result<T, failure::Error>;

pub struct MediaInfo {
    pub url: Url,
    pub title: Option<String>,
    pub referrer: Option<String>,
    pub user_agent: Option<String>,
}

impl MediaInfo {
    pub fn new(url: Url, title: Option<String>, referrer: Option<String>, user_agent: Option<String>,) -> Self {
        Self { url, title, referrer, user_agent }
    }
    pub fn default_ua(url: Url, title: Option<String>, referrer: Option<String>) -> Self {
        Self::new(url, title, referrer, Some(String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.132 Safari/537.36")))
    }
    pub fn play(&self) -> Res<()> {
        self.as_command()?.output()?;
        Ok(())
    }
    pub fn as_command(&self) -> Res<process::Command> {
        let Url { videos, audios } = &self.url;
        let mut cmd = process::Command::new("mpv");
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
            cmd.arg("--force-window=yes");
        } else {
            return Err(err_msg(format!("No urls to play")));
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
        Ok(cmd)
    }
}

pub fn parse(url: &str, pxy: &Option<proxy::ProxyAddr>) -> Res<MediaInfo> {
    find_parser!(url, bilibili, Annie, pxy);
    find_parser!(url, bilibili, YouGet, pxy);
    find_parser!(url, youtube, Annie, pxy);
    find_parser!(url, iqiyi, Annie, pxy);
    find_parser!(url, iqiyi, YouGet, pxy);
    Err(err_msg("Unsupport url"))
}