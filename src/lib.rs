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

pub mod cli;
pub mod command;
pub mod extractors;
pub mod parsers;
pub mod proxy;

use crate::cli::Config;
use crate::parsers::Url;
use anyhow::Result;
use finata::{Finata, Track};
use std::io::Result as IoResult;
use std::process::Command;

type ResultInfo = Result<MediaInfo>;

#[derive(Debug)]
pub struct MediaInfo {
    pub url: Url,
    pub title: Option<String>,
    pub referrer: Option<String>,
    pub user_agent: Option<String>,
}

impl MediaInfo {
    pub fn new(url: Url, title: Option<String>, referrer: Option<String>) -> Self {
        Self {
            url,
            title,
            referrer,
            user_agent: None,
        }
    }
    pub fn with_ua(
        url: Url,
        title: Option<String>,
        referrer: Option<String>,
        user_agent: String,
    ) -> Self {
        Self {
            url,
            title,
            referrer,
            user_agent: Some(user_agent),
        }
    }
    pub fn default_ua(url: Url, title: Option<String>, referrer: Option<String>) -> Self {
        Self::with_ua(url, title, referrer, String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/72.0.3579.1 Safari/537.36"))
    }
    pub fn play(&self, config: &Config) -> IoResult<()> {
        self.as_command(config).output()?;
        Ok(())
    }
    /// Spwans commands to run mpv
    pub fn as_command(&self, config: &Config) -> Command {
        let Url { videos, audios } = &self.url;
        let mut cmd = Command::new("mpv");
        match videos.len() {
            0 => cmd.args(audios.iter()).arg("--force-window=immediate"),
            1 => cmd
                .arg(&videos[0])
                .args(audios.iter().map(|a| format!("--audio-file={}", a))),
            _ => cmd
                .args(videos.iter())
                .args(audios.iter().map(|a| format!("--audio-file={}", a)))
                .args::<&[&str], _>(if config.merge {
                    &["--merge-files"]
                } else {
                    &[]
                }),
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
        if config.no_audio {
            cmd.args(&["--ao=null", "--no-audio"]);
        }
        if config.no_video {
            cmd.args(&["--no-video", "--force-window=immediate"]);
        }
        if let Some(proxy) = &config.proxy {
            cmd.env("HTTP_PROXY", proxy.to_string());
        }
        if let Some(cookie) = &config.cookie {
            cmd.arg(format!("--cookies-file={}", cookie));
        }
        if let Some(values) = config.commands.clone() {
            cmd.args(values);
        }
        cmd.arg("--no-ytdl");
        cmd
    }
}

pub fn spwan_command(playlist: Finata, config: &Config) -> Command {
    let mut cmd = Command::new("mpv");
    let orig = &playlist.raws()[0];
    let videos: Vec<_> = orig
        .tracks
        .iter()
        .filter(|t| matches!(t, Track::Video(_)))
        .collect();
    let audios = orig.tracks.iter().filter(|t| matches!(t, Track::Audio(_)));
    match videos.len() {
        0 => cmd.args(audios.map(|a| a.as_url().to_string())),
        1 => cmd
            .arg(videos[0].as_url().to_string())
            .args(audios.map(|a| format!("--audio-file={}", a.as_url().to_string()))),
        _ => cmd
            .args(videos.iter().map(|v| v.as_url().to_string()))
            .args(audios.map(|a| format!("--audio-file={}", a.as_url().to_string())))
            .args::<&[&str], _>(if config.merge {
                &["--merge-files"]
            } else {
                &[]
            }),
    };
    cmd.arg(playlist.title()).arg("--no-ytdl");
    if config.no_audio {
        cmd.args(&["--ao=null", "--no-audio"]);
    }
    if config.no_video {
        cmd.args(&["--no-video", "--force-window=immediate"]);
    }
    if let Some(proxy) = &config.proxy {
        cmd.env("HTTP_PROXY", proxy.to_string());
    }
    if let Some(cookie) = &config.cookie {
        cmd.arg(format!("--cookies-file={}", cookie));
    }
    if let Some(values) = config.commands.clone() {
        cmd.args(values);
    }
    cmd
}
