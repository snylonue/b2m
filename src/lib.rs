pub mod extractors;
pub mod cmd;

use serde_json;
use serde_json::Value;
use failure::err_msg;
use std::process;
use extractors::Url;

macro_rules! value_to_option_string {
    ($v: expr) => {
        match $v {
            serde_json::Value::String(ref s) => Some(s.clone()),
            _ => None,
        }
    };
}
macro_rules! parse_json {
    ($s: expr) => {
        match serde_json::from_str($s) {
            Ok(v) => v,
            Err(e) => return Err(err_msg(format!("Failed to deserialize stdout: {}", e))),
        }
    };
    ($s: expr, $err_msg: expr) => {
        match serde_json::from_str($s) {
            Ok(v) => v,
            Err(_) => return Err($err_msg),
        }
    };
}

type Res<T> = Result<T, failure::Error>;

pub enum Parser {
    YouGet,
    Annie,
}
pub struct MediaInfo {
    pub url: Url,
    pub title: Option<String>,
    pub referrer: Option<String>,
}

impl Parser {
    pub fn parse(&self, url: &str) -> Res<MediaInfo> {
        match *self {
            Parser::YouGet => {
                let (stdout, _) = cmd::run_command(process::Command::new("you-get")
                    .arg(url)
                    .arg("--json")
                    .stderr(process::Stdio::null())
                )?;
                let json_stdout = match serde_json::from_str(&stdout) {
                    Ok(j) => j,
                    Err(e) => return Err(err_msg(format!("Failed to deserialize stdout: {}", e))),
                };
                Self::you_get(&json_stdout)
            },
            Parser::Annie => {
                let (stdout, _) = cmd::run_command(process::Command::new("annie")
                    .arg("-j")
                    .arg(url)
                )?;
                //println!("{}", stdout);
                let json_output = parse_json!(&stdout);
                Self::annie(&json_output)
            }
        }
    }
    fn you_get(json: &Value) -> Res<MediaInfo> {
        let url = match extractors::parse_you_get_url(&json) {
            Some(el) => el,
            None => return Err(err_msg("Failed to parse stdout as url")),
        };
        // referrer = json['extra']['referer'] || json_output['url']
        let referrer = json.get("extra")
            .and_then(|v| v.get("referer"))
            .and_then(|v| v.as_str().and_then(to_option_string))
            .or_else(|| json["url"].as_str().and_then(to_option_string));
        // title = json['title']
        let title = json["title"].as_str().and_then(to_option_string);
        Ok(MediaInfo { url, referrer, title })
    }
    fn annie(json: &Value) -> Res<MediaInfo> {
        let url = match extractors::parse_annie_url(&json) {
            Some(el) => el,
            None => return Err(err_msg("Failed to parse stdout as url")),
        };
        let referrer = value_to_option_string!(json["url"]);
        let title = value_to_option_string!(json["title"]);
        Ok(MediaInfo { url, referrer, title })
    }
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
                cmd.arg("--force-window");
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