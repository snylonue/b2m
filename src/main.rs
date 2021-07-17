#[allow(unused_macros)]
macro_rules! features {
    ($($feature: tt),*) => {
        println!("features enabled:");
        $(#[cfg(feature = $feature)]
        println!(concat!("  ", $feature));)*
    };
}

mod check;

use anyhow::Result;
use b2m::*;
use finata::website::bilibili::Bangumi;
use finata::website::bilibili::Video;
use finata::website::netease_music::PlayList;
use finata::website::netease_music::Song;
use finata::website::pixiv::Collection;
use finata::website::pixiv::Pixiv;
use finata::Config;
use finata::Extract;
use finata::ExtractSync;
use netscape_cookie::parse;
use std::fs::File;
use std::io::Read;

fn main() -> Result<()> {
    let matches = cli::b2m().get_matches();
    let config = cli::Config::new(&matches)?;
    if config.check {
        check(&config);
        return Ok(());
    }
    let mut extractor = find_extractor(config.url)?;
    if let Some(path) = config.cookie {
        let mut cookie_file = File::open(path)?;
        let mut buf = Vec::new();
        cookie_file.read_to_end(&mut buf)?;
        let cookies: Vec<_> = parse(&buf)?
            .iter()
            .map(|cookie| format!("{}={}", cookie.name, cookie.value))
            .collect();
        extractor.client_mut().push_cookie(&cookies.join(";"))?;
    }
    let res = extractor.extract_sync()?;
    spwan_command(res, &config).spawn()?;
    Ok(())
}
fn check(conf: &cli::Config) {
    println!("Running check");
    println!("b2m version: {}\n", cli::VERSION);
    dbg!(conf);
    if check::check_mpv() {
        println!("\nmpv check succeeded");
    } else {
        println!("\nmpv check failed");
    }
}
trait Extractor: Extract + Config {}

impl<T: Extract + Config> Extractor for T {}

fn find_extractor(url: &str) -> Result<Box<dyn Extractor>> {
    if let Ok(extr) = Video::new(url) {
        return Ok(Box::new(extr));
    }
    if let Ok(extr) = Bangumi::new(url) {
        return Ok(Box::new(extr));
    }
    if let Ok(extr) = Song::new(url.parse()?) {
        return Ok(Box::new(extr));
    }
    if let Ok(extr) = PlayList::new(url.parse()?) {
        return Ok(Box::new(extr));
    }
    if let Ok(extr) = Pixiv::new(url) {
        return Ok(Box::new(extr));
    }
    if let Ok(extr) = Collection::new(url) {
        return Ok(Box::new(extr));
    }
    Err(anyhow::anyhow!("Unsupported url"))
}
