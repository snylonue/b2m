pub mod cli;
pub mod command;
pub mod proxy;

use crate::cli::Config;
use finata::{Finata, Origin, Track};
use std::process::Command;

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
    cmd.arg(format!("--title={}", playlist.title()))
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
