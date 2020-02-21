use clap::Arg;
use clap::App;
use clap::SubCommand;

pub const NAME: &str = "mpv-bilibili";
pub const VERSION: &str = "0.13.0";
pub const DESCRIPTION: &str = "Play bilibili video with mpv";

#[inline]
pub fn b2m() -> App<'static, 'static> {
    App::new(NAME)
        .version(VERSION)
        .about(DESCRIPTION)
        .arg(Arg::with_name("url")
            .help("Video url")
            .index(1)
            .required_unless("check")
    )
        .arg(Arg::with_name("check")
            .help("Check if all dependencies are installed")
            .short("c")
            .long("check")
            .multiple(true)
    )
        .arg(Arg::with_name("no-audio")
            .help("Play without audio output")
            .long("an")
            .alias("no-audio")
            .multiple(true)
    )
        .arg(Arg::with_name("no-video")
            .help("Play without video output")
            .long("vn")
            .alias("no-video")
            .multiple(true)
    )
        .arg(Arg::with_name("info-only")
            .help("Print information only")
            .long("info")
            .short("i")
            .multiple(true)
            .group("info")
    )
        .arg(Arg::with_name("json")
            .help("Print stdout in json")
            .long("json")
            .short("j")
            .multiple(true)
            .group("info")
    )
}
#[inline]
#[allow(dead_code)]
pub fn mpv() -> App<'static, 'static> {
    SubCommand::with_name("mpv")
        .about("mpv configuration")
        .arg(Arg::with_name("mpv-conf"))
}