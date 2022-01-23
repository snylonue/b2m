mod check;

use anyhow::Result;
#[cfg(feature = "fina")]
use b2m::parsers::fina::Fina;
use b2m::*;

fn main() -> Result<()> {
    let matches = cli::b2m().get_matches();
    let config = cli::Config::new(&matches)?;
    if config.check {
        check(&config);
        return Ok(());
    }
    let mut extractor = find_extractor(&config)?;
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

fn find_extractor(conf: &cli::Config) -> Result<Box<dyn Extractor>> {
    let url = conf.url;
    if let Ok(extr) = Fina::new(url) {
        return Ok(Box::new(extr));
    }
    // todo: check whether lux supports this url
    Ok(Box::new(b2m::parsers::lux::Lux::new(url)))
}
