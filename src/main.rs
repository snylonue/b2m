pub mod check;

use clap::Arg;
use clap::App;
use failure::Error;
use std::process;

use b2m::*;

const NAME: &str = "mpv-bilibili";
const VERSION: &str = "0.10.2";
const DESCRIPTION: &str = "play bilibili video with mpv";

fn main() -> Result<(), Error> {
    let matches = App::new(NAME)
        .version(VERSION)
        .about(DESCRIPTION)
        .arg(Arg::with_name("url")
            .help("video url")
            .index(1)
            .required_unless("check")
    )
        .arg(Arg::with_name("check")
            .help("check if all dependencies are installed")
            .short("c")
            .long("check")
            .multiple(true)
    )
        .arg(Arg::with_name("no-audio")
            .help("play without audio output")
            .long("an")
            .multiple(true)
            .conflicts_with("no-video")
    )
        .arg(Arg::with_name("url-only")
            .help("print url only")
            .long("url")
            .short("u")
            .multiple(true)
    )
        .get_matches();
    if matches.is_present("check") {
        check();
        process::exit(0);
    }
    let url = matches.value_of("url").expect("Invaild input");
    let mut media = get_url(&url.into())?;
    if matches.is_present("no-audio") {
        media.url.audios = vec![];
    }
    if matches.is_present("url-only") {
        print_info(media, matches.is_present("json"));
        process::exit(0);
    }
    media.play()
}
fn check() {
    if check::check_you_get() {
        println!("\nyou-get checking succeeded");
    } else {
        println!("\nyou-get checking failed");
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
        // pass
    } else {
        println!("video: {:#?}", videos);
        println!("audio: {:#?}", audios);
        println!("title: {}", title.unwrap_or(String::new()));
        println!("referrer: {}", referrer.unwrap_or(String::new()));
    }

}