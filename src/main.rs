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
use finata::Extract;
use finata::ExtractSync;

use b2m::*;
use finata::website::bilibili::Bangumi;
use finata::website::bilibili::Video;
use finata::website::netease_music::PlayList;
use finata::website::netease_music::Song;
use finata::website::pixiv::Collection;
use finata::website::pixiv::Pixiv;

fn main() -> Result<()> {
    let matches = cli::b2m().get_matches();
    let config = cli::Config::new(&matches)?;
    if config.check {
        check(&config);
        return Ok(());
    }
    let mut extractor = find_extractor(config.url)?;
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
fn find_extractor(url: &str) -> Result<Box<dyn Extract>> {
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
