mod check;
mod cli;

use failure::Error;
use serde_json::json;
use std::process;

use b2m::*;

fn main() -> Result<(), Error> {
    let matches = cli::b2m().get_matches();
    if matches.is_present("check") {
        check();
        process::exit(0);
    }
    let url = matches.value_of("url").expect("Invaild input");
    let media = parse(url)?;
    if matches.is_present("info-only") {
        print_info(media, matches.is_present("json"));
        process::exit(0);
    }
    let mut commands = media.as_command()?;
    if matches.is_present("no-audio") {
        commands.arg("--ao=null");
    }
    // --vn doesn't work well
    if matches.is_present("no-video") {
        commands.arg("--vo=null");
        commands.arg("--force-window=immediate");
    }
    println!("{:?}", commands);
    commands.output()?;
    Ok(())
}
fn check() {
    println!("Running checking");
    println!("b2m version: {}\n", cli::VERSION);
    if check::check_you_get() {
        println!("\nyou-get checking succeeded");
    } else {
        println!("\nyou-get checking failed");
    }
    println!();
    if check::check_annie() {
        println!("\nannie checking succeeded");
    } else {
        println!("\nannie checking failed");
    }
    println!();
    if check::check_mpv() {
      println!("\nmpv checking succeeded");
    } else {
      println!("\nmpv checking failed");
    }
}
fn print_info(media: MediaInfo, json: bool) {
    let MediaInfo { url: extractors::Url { videos, audios }, title, referrer } = media;
    if json {
        let j = json!({
            "video": videos,
            "audio": audios,
            "title": title,
            "referrer": referrer,
        });
        println!("{}", j.to_string());
    } else {
        println!("video: {}", serde_json::to_string(&videos).unwrap());
        println!("audio: {:#?}", serde_json::to_string(&audios).unwrap());
        println!("title: {}", title.unwrap_or_else(|| String::new()));
        println!("referrer: {}", referrer.unwrap_or_else(|| String::new()));
    }
}
