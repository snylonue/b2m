use clap::Arg;
use clap::App;
use clap::SubCommand;
use clap::ArgMatches;

pub const NAME: &str = "mpv-bilibili";
pub const VERSION: &str = "0.14.0";
pub const DESCRIPTION: &str = "Play bilibili video with mpv";

pub struct Config<'a> {
    pub url: &'a str,
    pub check: bool,
    pub no_audio: bool,
    pub no_video: bool,
    pub info: bool,
    pub json: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a ArgMatches) -> Self {
        let url = args.value_of("url").expect("Invaild input");
        let check = args.is_present("check");
        let mut info = args.is_present("info-only");
        let json = args.is_present("json");
        let no_audio = args.is_present("no-audio");
        let no_video = args.is_present("no-video");
        if json {
            info = true;
        }
        Self { url, check, no_audio, no_video, info, json }
    }
}

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
    )
        .arg(Arg::with_name("json")
            .help("Print stdout in json")
            .long("json")
            .short("j")
            .multiple(true)
    )
}
#[inline]
#[allow(dead_code)]
pub fn mpv() -> App<'static, 'static> {
    SubCommand::with_name("mpv")
        .about("mpv configuration")
        .arg(Arg::with_name("mpv-conf"))
}