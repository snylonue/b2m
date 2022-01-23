mod check;

use anyhow::Result;
use b2m::*;
#[cfg(feature = "fina")]
mod fina {
    pub use finata::website::bilibili::Bangumi;
    pub use finata::website::bilibili::Video;
    pub use finata::website::netease_music::PlayList;
    pub use finata::website::netease_music::Song;
    pub use finata::website::pixiv::Collection;
    pub use finata::website::pixiv::Pixiv;
}
#[cfg(feature = "fina")]
use fina::*;

fn main() -> Result<()> {
    let matches = cli::b2m().get_matches();
    let config = cli::Config::new(&matches)?;
    if config.check {
        check(&config);
        return Ok(());
    }
    let mut extractor = find_extractor(config.url)?;
    if let Some(path) = config.cookie {
        extractor.load_netscape_cookie(path.as_ref())?;
    }
    let res = extractor.extract()?;
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

fn find_extractor(url: &str) -> Result<Box<dyn Extractor>> {
    #[cfg(feature = "fina")]
    if let Ok(extr) = Video::new(url) {
        return Ok(Box::new(extr));
    }
    #[cfg(feature = "fina")]
    if let Ok(extr) = Bangumi::new(url) {
        return Ok(Box::new(extr));
    }
    #[cfg(feature = "fina")]
    if let Ok(extr) = Song::new(url) {
        return Ok(Box::new(extr));
    }
    #[cfg(feature = "fina")]
    if let Ok(extr) = PlayList::new(url) {
        return Ok(Box::new(extr));
    }
    #[cfg(feature = "fina")]
    if let Ok(extr) = Pixiv::new(url) {
        if url.contains("pixiv") {
            return Ok(Box::new(extr));
        }
    }
    #[cfg(feature = "fina")]
    if let Ok(extr) = Collection::new(url) {
        if url.contains("pixiv") {
            return Ok(Box::new(extr));
        }
    }
    // todo: check whether lux supports this url
    Ok(Box::new(b2m::parsers::lux::Lux::new(url)))
}
