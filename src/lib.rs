pub mod cli;
pub mod command;
pub mod parsers;
pub mod proxy;

use crate::cli::Config;
use anyhow::{Result, anyhow};
use finata::{ExtractSync, Finata, Origin, Track};
use std::{path::Path, process::Command};

pub trait Extractor {
    fn extract(&mut self) -> Result<Finata>;
    fn load_netscape_cookie(&mut self, cookie: &Path) -> Result<()>;
}

impl Extractor for Box<dyn finata::website::Extractor> {
    fn extract(&mut self) -> Result<Finata> {
        Ok(self.extract_sync()?)
    }
    fn load_netscape_cookie(&mut self, cookie: &Path) -> Result<()> {
        Ok(self.client_mut().load_netscape_cookie(cookie)?)
    }
}

impl Extractor for Vec<Box<dyn Extractor>> {
    fn extract(&mut self) -> Result<Finata> {
        let mut res = None;
        for ex in self {
            res = Some(ex.extract());
            if let Some(Ok(_)) = res {
                return res.unwrap();
            }
        }
        res.unwrap_or(Err(anyhow!("No extractor")))
    }

    fn load_netscape_cookie(&mut self, cookie: &Path) -> Result<()> {
        let mut res = Ok(());
        for ex in self {
            res = ex.load_netscape_cookie(cookie);
        }
        res
    }
}

fn push_media(media: &Origin, cmd: &mut Command, config: &Config) {
    cmd.arg("--{");
    let videos: Vec<_> = media
        .tracks
        .iter()
        .filter(|t| matches!(t, Track::Video(_)))
        .collect();
    let audios = media.tracks.iter().filter(|t| matches!(t, Track::Audio(_)));
    match videos.len() {
        0 => cmd.args(audios.map(|a| a.as_url().to_string())),
        1 => cmd
            .arg(videos[0].as_url().to_string())
            .args(audios.map(|a| format!("--audio-file={}", a.as_url().to_string()))),
        _ => cmd
            .args(videos.iter().map(|v| v.as_url().to_string()))
            .args(audios.map(|a| format!("--audio-file={}", a.as_url().to_string())))
            .args::<&[&str], _>(if config.merge {
                // currently doesn't work due to https://github.com/mpv-player/mpv/issues/9522
                &["--merge-files"]
            } else {
                &[]
            }),
    };
    cmd.arg(format!("--referrer={}", config.url)).arg("--}");
}
pub fn spwan_command(playlist: Finata, config: &Config) -> Command {
    let mut cmd = Command::new("mpv");
    for media in playlist.raws() {
        push_media(media, &mut cmd, config);
    }
    cmd.arg(format!("--force-media-title={}", playlist.title()))
        .arg("--no-ytdl");
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
