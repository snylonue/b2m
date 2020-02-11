macro_rules! find_parser {
    ($url: expr, $site: ident, $parser: ident) => {
        if $crate::extractors::$site::$parser::is_support($url) {
            return $crate::extractors::$site::$parser::parse($url);
        }
    };
}

pub mod extractors;
pub mod cmd;

use failure::err_msg;
use std::process;
use extractors::Url;
use extractors::Parser;

type Res<T> = Result<T, failure::Error>;

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
            if videos.len() > 1 {
                cmd.arg("--merge-files");
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
        cmd.arg("--no-ytdl")
            .output()?;
        Ok(())
    }
}