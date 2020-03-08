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
#[macro_export]
macro_rules! parse_json {
    ($s: expr) => {
        match serde_json::from_str($s) {
            Ok(v) => v,
            Err(e) => return Err(failure::err_msg(format!("Failed to deserialize: {}", e))),
        }
    };
    ($s: expr, $err_msg: expr) => {
        match serde_json::from_str($s) {
            Ok(v) => v,
            Err(_) => return Err($err_msg),
        }
    };
}
macro_rules! find_parser {
    ($url: expr, $site: ident, $extractor: ident) => {
        if $crate::extractors::$site::$extractor::is_support($url) {
            return $crate::extractors::$site::$extractor::extract($url);
        }
    };
}

pub mod proxy;
pub mod cmd;
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
}

impl MediaInfo {
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
        cmd.arg("--no-ytdl");
        Ok(cmd)
    }
}

pub fn parse(url: &str) -> Res<MediaInfo> {
    find_parser!(url, bilibili, Annie);
    find_parser!(url, bilibili, YouGet);
    find_parser!(url, iqiyi, Annie);
    find_parser!(url, iqiyi, YouGet);
    Err(err_msg("Unsupport url"))
}