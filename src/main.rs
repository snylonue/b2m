macro_rules! find_parser {
    ($url: expr, $setting: expr, $($site: ident, $extractor_name: expr, $extractor: ident),*) => {
       {
            use $crate::extractors::Extractor;
            $(#[cfg(feature = $extractor_name)]if $crate::extractors::$site::$extractor::is_support($url) {
                match $crate::extractors::$site::$extractor::extract($url, $setting) {
                    res @ Ok(_) => return res,
                    Err(e) => { 
                        eprintln!("Error caught with {}/{}", stringify!($site), $extractor_name);
                        eprintln!("Error: {:#?}", e);
                     }
                }
            })*
        }
    };
}

mod check;

use anyhow::Result;
use serde_json::json;

use b2m::*;

fn main() -> Result<()> {
    let matches = cli::b2m().get_matches();
    let config = cli::Config::new(&matches)?;
    if config.check {
        check();
        return Ok(());
    }
    let settings = Setting::new(config.proxy, config.cookie);
    let media = parse(config.url, &config)?;
    if config.info {
        print_info(media, config.json);
        return Ok(());
    }
    let mut commands = media.as_command();
    if config.no_audio {
        commands.arg("--ao=null")
            .arg("--no-audio");
    }
    if config.no_video {
        commands.arg("--no-video")
            .arg("--force-window=immediate");
    }
    if let Some(proxy) = &settings.proxy_addr {
        commands.env("HTTP_PROXY", proxy.to_string());
    }
    if let Some(cookie) = &settings.cookie {
        commands.arg(format!("--cookies-file={}", cookie));
    }
    commands.output()?;
    Ok(())
}
fn check() {
    println!("Running check");
    println!("b2m version: {}\n", cli::VERSION);
    if check::check_you_get() {
        println!("\nyou-get check succeeded");
    } else {
        println!("\nyou-get check failed");
    }
    println!();
    if check::check_annie() {
        println!("\nannie check succeeded");
    } else {
        println!("\nannie check failed");
    }
    println!();
    if check::check_mpv() {
      println!("\nmpv check succeeded");
    } else {
      println!("\nmpv check failed");
    }
}
fn print_info(media: MediaInfo, json: bool) {
    let MediaInfo { url: parsers::Url { videos, audios }, title, referrer, user_agent } = media;
    if json {
        let j = json!({
            "video": videos,
            "audio": audios,
            "title": title,
            "referrer": referrer,
            "user-agent": user_agent,
        });
        println!("{}", serde_json::to_string_pretty(&j).unwrap());
    } else {
        println!("video: {}", serde_json::to_string(&videos).unwrap());
        println!("audio: {}", serde_json::to_string(&audios).unwrap());
        println!("title: {}", title.unwrap_or_else(String::new));
        println!("referrer: {}", referrer.unwrap_or_else(String::new));
        println!("user-agent: {}", user_agent.unwrap_or_else(String::new));
    }
}
pub fn parse(url: &str, setting: &cli::Config) -> Result<MediaInfo> {
    find_parser!(
        url, setting,
        bilibili, "annie", Annie,
        bilibili, "youget", YouGet,
        youtube, "annie", Annie,
        iqiyi, "annie", Annie,
        iqiyi, "youget", YouGet,
        netease_music, "nfinata", Finata
    );
    Err(anyhow::anyhow!("Unsupport url"))
}