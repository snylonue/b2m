pub mod cli;
pub mod command;
pub mod parsers;
pub mod proxy;

use crate::cli::Config;
use anyhow::{anyhow, Result};
use finata::{Finata, Origin, Track};
use std::{path::Path, process::Command};

pub trait Extractor {
    fn name(&self) -> &'static str {
        "unknown"
    }
    fn extract(&mut self) -> Result<Finata>;
    fn load_cookie(&mut self, cookie: &Path) -> Result<()>;
}

impl Extractor for Vec<Box<dyn Extractor>> {
    fn name(&self) -> &'static str {
        "finata"
    }
    fn extract(&mut self) -> Result<Finata> {
        let mut res = None;
        for ex in self {
            res = Some(ex.extract());
            match res {
                Some(r @ Ok(_)) => return r,
                Some(Err(ref e)) => eprintln!("Extractor error({}): {}", ex.name(), e),
                _ => {}
            }
        }
        res.unwrap_or_else(|| Err(anyhow!("No extractor")))
    }

    fn load_cookie(&mut self, cookie: &Path) -> Result<()> {
        for ex in self {
            match ex.load_cookie(cookie) {
                Err(e) => eprintln!("Fails to load cookie({}): {}", ex.name(), e),
                _ => {}
            }
        }
        Ok(())
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
            .args(audios.map(|a| format!("--audio-file={}", a.as_url()))),
        _ => cmd
            .args(videos.iter().map(|v| v.as_url().to_string()))
            .args(audios.map(|a| format!("--audio-file={}", a.as_url())))
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
