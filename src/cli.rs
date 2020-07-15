use anyhow::Result;
use clap::Arg;
use clap::App;
use clap::ArgMatches;
use crate::proxy::ProxyAddr;

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

#[derive(Debug)]
pub struct Config<'a> {
    pub url: &'a str,
    pub check: bool,
    pub no_audio: bool,
    pub no_video: bool,
    pub info: bool,
    pub json: bool,
    pub proxy: Option<ProxyAddr<'a>>,
    pub cookie: Option<&'a str>,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a ArgMatches) -> Result<Self> {
        let check = args.is_present("check");
        let url = if !check {
            args.value_of("url").expect("Invaild input")
        } else {
            ""
        };
        let json = args.is_present("json");
        let info = json || args.is_present("info-only");
        let no_audio = args.is_present("no-audio");
        let no_video = args.is_present("no-video");
        let proxy = match args.value_of("proxy") {
            Some(p) if args.occurrences_of("proxy") == 1 => Some(ProxyAddr::from_str(p)?),
            _ => None,
        };
        let cookie = args.value_of("cookie");
        Ok(Self { url, check, no_audio, no_video, info, json, proxy, cookie })
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
            // .short("c")
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
            .help("Print information with json")
            .long("json")
            .short("j")
            .multiple(true)
    )
        .arg(Arg::with_name("proxy")
            .help("Set proxy address")
            .long("proxy")
            .short("p")
            .default_value("127.0.0.1:1080")
    )
        .arg(Arg::with_name("cookie")
            .help("Set cookie")
            .long("cookie")
            .short("c")
            .takes_value(true)
    )
}