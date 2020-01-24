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
fn to_option_string(s: &str) -> Option<String> {
    Some(s.to_string())
}
pub fn get_origin_url(json: &str) -> Res<MediaInfo> {
    let json_stdout = match serde_json::from_str(json) {
            Ok(j) => j,
            Err(e) => return Err(err_msg(format!("Failed to deserialize stdout: {}", e))),
    };
    let url = match parse_url(&json_stdout) {
        Some(el) => el,
        None => return Err(err_msg("Failed to parse stdout as url")),
    };
    // referrer = json_output['extra']['referer'] || json_output['url']
    let referrer = json_stdout.get("extra")
        .and_then(|v| { v.get("referer") })
        .and_then(|v| { v.as_str().and_then(to_option_string) })
        .or_else(|| { json_stdout["url"].as_str().and_then(to_option_string) });
    // title = json_output['title']
    let title = json_stdout["title"].as_str().and_then(to_option_string);
    Ok(MediaInfo { url, referrer, title })
}