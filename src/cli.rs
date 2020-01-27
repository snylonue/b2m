use clap::Arg;
use clap::App;
use clap::SubCommand;

pub const NAME: &str = "mpv-bilibili";
pub const VERSION: &str = "0.10.3";
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
            .multiple(true)
            .conflicts_with("no-video")
    )
        .arg(Arg::with_name("url-only")
            .help("Print url only")
            .long("url")
            .short("u")
            .multiple(true)
    )
        .arg(Arg::with_name("json")
            .help("Print stdout in json")
            .long("json")
            .short("j")
            .multiple(true)
            .requires("url-only")
    )
}
#[inline]
#[allow(dead_code)]
pub fn mpv() -> App<'static, 'static> {
    SubCommand::with_name("mpv")
        .about("mpv configuration")
        .arg(Arg::with_name("mpv-conf"))
}