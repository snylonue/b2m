use crate::proxy::ProxyAddr;
use anyhow::Result;
use clap::App;
use clap::Arg;
use clap::ArgMatches;
use clap::Values;

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const DEFAULT_COOKIES: Option<&str> = option_env!("B2M_DEFAULT_COOKIES");
pub const DEFAULT_PROXY: Option<&str> = option_env!("B2M_DEFAULT_PROXY");

#[derive(Debug, Default, Clone)]
pub struct Config<'a> {
    pub url: &'a str,
    pub check: bool,
    pub no_audio: bool,
    pub no_video: bool,
    pub info: bool,
    pub json: bool,
    pub proxy: Option<ProxyAddr>,
    pub cookie: Option<&'a str>,
    pub merge: bool,
    pub parser: Option<&'a str>,
    pub commands: Option<Values<'a>>,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a ArgMatches) -> Result<Self> {
        let check = args.is_present("check");
        let url = args.value_of("url").unwrap_or_default();
        let json = args.is_present("json");
        let info = json || args.is_present("info-only");
        let no_audio = args.is_present("no-audio");
        let no_video = args.is_present("no-video");
        let proxy = match args.value_of("proxy") {
            Some(p) if args.occurrences_of("proxy") == 1 => Some(p.parse()?),
            _ => None,
        };
        let cookie = if !args.is_present("no-cookie") {
            args.value_of("cookie").or(DEFAULT_COOKIES)
        } else {
            None
        };
        let commands = args.values_of("mpv-args");
        let parser = args.value_of("parser");
        let merge = !args.is_present("no-merging");
        Ok(Self {
            url,
            check,
            no_audio,
            no_video,
            info,
            json,
            proxy,
            cookie,
            merge,
            parser,
            commands,
        })
    }
}

#[inline]
pub fn b2m() -> App<'static, 'static> {
    App::new(NAME)
        .version(VERSION)
        .about(DESCRIPTION)
        .arg(
            Arg::with_name("url")
                .help("Video url")
                .index(1)
                .required_unless("check"),
        )
        .arg(
            Arg::with_name("check")
                .help("Check if all dependencies are installed")
                .long("check")
                .multiple(true),
        )
        .arg(
            Arg::with_name("no-audio")
                .help("Play without audio output")
                .long("an")
                .alias("no-audio")
                .alias("vo")
                .multiple(true),
        )
        .arg(
            Arg::with_name("no-video")
                .help("Play without video output")
                .long("vn")
                .alias("no-video")
                .alias("ao")
                .multiple(true),
        )
        .arg(
            Arg::with_name("info-only")
                .help("Print information only")
                .long("info")
                .short("i")
                .multiple(true),
        )
        .arg(
            Arg::with_name("json")
                .help("Print information with json")
                .long("json")
                .short("j")
                .multiple(true),
        )
        .arg(
            Arg::with_name("proxy")
                .help("Set proxy address")
                .long("proxy")
                .short("p")
                .default_value(DEFAULT_PROXY.unwrap_or("127.0.0.1:1080"))
                .env("HTTP_PROXY"),
        )
        .arg(
            Arg::with_name("cookie")
                .help("Load cookie")
                .long("cookie")
                .short("c")
                .takes_value(true)
                .env("B2M_COOKIES"),
        )
        .arg(
            Arg::with_name("no-cookie")
                .help("Don't use any cookie")
                .long("no-cookie")
                .alias("nc"),
        )
        .arg(
            Arg::with_name("no-merging")
                .help("Don't pass --merge-files to mpv")
                .long("no-merge")
                .alias("nm"),
        )
        .arg(
            Arg::with_name("parser")
                .help("Choose a parser")
                .long("parser")
                .takes_value(true)
                .possible_values(&["lux", "fina"]),
        )
        .arg(
            Arg::with_name("mpv-args")
                .help("args to pass to mpv, may have some limitations")
                .takes_value(true)
                .raw(true),
        )
}
