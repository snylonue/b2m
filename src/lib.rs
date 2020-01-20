pub mod extractors;
pub mod cmd;

use serde_json;
use serde_json::Value;
use failure::err_msg;
use failure::Error;
use std::process;
use std::panic;
use extractors::parse_url;
use extractors::Url;

type Res<T> = Result<T, Error>;

pub struct MediaInfo {
    pub url: Url,
    pub title: Option<String>,
    pub referrer: Option<String>,
}

impl MediaInfo {
    pub fn play(&self, vo: bool, ao: bool) -> Res<()> {
        let Url { videos, audios } = &self.url;
        let mut cmd = process::Command::new("mpv");
        if vo && videos.len() > 0 {
            for i in videos {
                cmd.arg(i);
            }
            if ao {
                for i in audios {
                    cmd.arg(format!("--audio-file={}", i));
                }
            }
        } else if ao && audios.len() > 0 {
            for i in audios {
                cmd.arg(i);
            }
        } else {
            return Err(err_msg(format!("No urls to play, no-video: {}, no-audio: {}",
                if vo { "no" } else { "yes" },
                if ao { "no" } else { "yes" }
            )))
        }
        if let Some(referrer) = &self.referrer {
            cmd.arg(format!("--referrer={}", referrer));
        }
        if let Some(title) = &self.title {
            cmd.arg(format!("--title={}", title));
        }
        cmd.arg("--merge-files")
            .arg("--no-ytdl")
            .output()?;
        Ok(())
    }
}

pub fn get_url(orig_url: &String) -> Res<MediaInfo> {
    let (stdout, _) = cmd::run_command(process::Command::new("you-get")
        .arg(orig_url)
        .arg("--json"))?;
    let json_stdout = match serde_json::from_str(&*stdout) {
            Ok(j) => j,
            Err(e) => return Err(err_msg(format!("Failed to deserialize stdout: {}", e))),
    };
    let url = match parse_url(&json_stdout) {
        Some(el) => el,
        None => return Err(err_msg("Failed to parse stdout as url")),
    };
    // referrer = json_output['extra']['referer']
    let referrer = panic::catch_unwind(|| {
        match json_stdout["extra"]["referer"] {
            Value::String(ref s) => Some(s.clone()),
            _ => Some(json_stdout["url"].as_str()?.to_string()),
        }
    }).unwrap_or(None);
    // title = json_output['title']
    let title = json_stdout["title"].as_str().and_then(|s| { Some(s.to_string()) });
    Ok(MediaInfo { url, referrer, title })
}