macro_rules! find_parser {
    ($url: expr, $setting: expr, $($site: ident, $extractor_name: expr, $extractor: ident),*) => {
       {
            use $crate::extractors::Extractor;
            $(#[cfg(feature = $extractor_name)]
            if $setting.parser.map(|p| p == $extractor_name).unwrap_or(true)
                && $crate::extractors::$site::$extractor::is_support($url) {
                if stringify!($extractor) == "Annie" {
                    eprintln!("Warning: annie has been renamed to lux and will be replaced by lux in the future");
                }
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
macro_rules! features {
    ($($feature: tt),*) => {
        println!("features enabled:");
        $(#[cfg(feature = $feature)]
        println!(concat!("  ", $feature));)*
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
        check(&config);
        return Ok(());
    }
    let media = parse(config.url, &config)?;
    if config.info {
        print_info(media, config.json);
        return Ok(());
    }
    Ok(media.play(&config)?)
}
fn check(conf: &cli::Config) {
    println!("Running check");
    println!("b2m version: {}\n", cli::VERSION);
    features!("annie", "nfinata", "youget");
    dbg!(conf);
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
    let MediaInfo {
        url: parsers::Url { videos, audios },
        title,
        referrer,
        user_agent,
    } = media;
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
#[rustfmt::skip]
pub fn parse(url: &str, setting: &cli::Config) -> Result<MediaInfo> {
    find_parser!(
        url, setting,
        bilibili, "nfinata", Finata,
        bilibili, "annie", Lux,
        bilibili, "annie", Annie,
        bilibili, "youget", YouGet,
        youtube, "annie", Annie,
        iqiyi, "annie", Annie,
        iqiyi, "youget", YouGet,
        netease_music, "nfinata", Finata
    );
    Err(anyhow::anyhow!("Unsupport url"))
}
