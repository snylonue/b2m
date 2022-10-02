use std::str::FromStr;

use crate::proxy::ProxyAddr;
use anyhow::Result;
use clap::parser::ValuesRef;
use clap::Arg;
use clap::ArgAction;
use clap::ArgMatches;
use clap::Command;

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
    pub proxy: Option<&'a ProxyAddr>,
    pub cookie: Option<&'a str>,
    pub merge: bool,
    pub parser: Option<&'a str>,
    pub commands: Option<ValuesRef<'a, String>>,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a ArgMatches) -> Result<Self> {
        let check = args.get_flag("check");
        let url = args
            .get_one::<String>("url")
            .map(|s| s.as_str())
            .unwrap_or_default();
        let json = args.get_flag("json");
        let info = json || args.get_flag("info-only");
        let no_audio = args.get_flag("no-audio");
        let no_video = args.get_flag("no-video");
        let proxy = args.get_one::<ProxyAddr>("proxy");
        let cookie = if !args.get_flag("no-cookie") {
            args.get_one::<String>("cookie")
                .map(|s| s.as_str())
                .or(DEFAULT_COOKIES)
        } else {
            None
        };
        let commands = args.get_many::<String>("mpv-args");
        let parser = args.get_one::<String>("parser").map(|s| s.as_str());
        let merge = !args.get_flag("no-merging");
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
pub fn b2m() -> Command {
    Command::new(NAME)
        .version(VERSION)
        .about(DESCRIPTION)
        .arg(
            Arg::new("url")
                .help("Video url")
                .num_args(1)
                .required_unless_present("check"),
        )
        .arg(
            Arg::new("check")
                .help("Check if all dependencies are installed")
                .long("check")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-audio")
                .help("Play without audio output")
                .long("an")
                .alias("no-audio")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-video")
                .help("Play without video output")
                .long("vn")
                .alias("no-video")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("info-only")
                .help("Print information only")
                .long("info")
                .short('i')
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("json")
                .help("Print information with json")
                .long("json")
                .short('j')
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("proxy")
                .help("Set proxy address")
                .long("proxy")
                .short('p')
                .num_args(0..=1)
                .default_missing_value(DEFAULT_PROXY.unwrap_or("127.0.0.1:1080"))
                .env("HTTP_PROXY")
                .value_parser(ProxyAddr::from_str),
        )
        .arg(
            Arg::new("cookie")
                .help("Load cookie")
                .long("cookie")
                .short('c')
                .num_args(1)
                .env("B2M_COOKIES"),
        )
        .arg(
            Arg::new("no-cookie")
                .help("Don't use any cookie")
                .long("no-cookie")
                .alias("nc")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-merging")
                .help("Don't pass --merge-files to mpv")
                .long("no-merge")
                .alias("nm")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("parser")
                .help("Choose a parser")
                .long("parser")
                .num_args(1)
                .value_parser(["fina", "lux"]),
        )
        .arg(
            Arg::new("mpv-args")
                .help("args to pass to mpv, may have some limitations")
                .raw(true),
        )
}
