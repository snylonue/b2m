pub mod extractors;
pub mod cmd;

use serde_json;
use failure::err_msg;
use failure::Error;
use std::process;
use extractors::parse_url;
use extractors::Url;

type Res<T> = Result<T, Error>;

pub struct MediaInfo {
    pub url: Url,
    pub title: Option<String>,
    pub referrer: Option<String>,
}

impl MediaInfo {
    pub fn play(&self) -> Res<()> {
        let Url { videos, audios } = &self.url;
        let mut cmd = process::Command::new("mpv");
        if videos.len() > 0 {
            for i in videos {
                cmd.arg(i);
            }
            for i in audios {
                cmd.arg(format!("--audio-file={}", i));
            }
        } else if audios.len() > 0 {
            for i in audios {
                cmd.arg(i);
            }
        } else {
            return Err(err_msg(format!("No urls to play")));
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

#[inline]
fn opt_to_string(s: &str) -> Option<String> {
    Some(s.to_string())
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
    let referrer = json_stdout.get("extra")
        .and_then(|v| { v.get("referer") })
        .and_then(|v| { v.as_str().and_then(opt_to_string) })
        .or_else(|| { json_stdout["url"].as_str().and_then(opt_to_string) });
    // title = json_output['title']
    let title = json_stdout["title"].as_str().and_then(opt_to_string);
    Ok(MediaInfo { url, referrer, title })
}