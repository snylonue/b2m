mod check;

use anyhow::Result;
use b2m::*;
#[cfg(feature = "fina")]
use parsers::fina::Fina;
use parsers::lux::Lux;
#[cfg(not(windows))]
use std::os::unix::process::CommandExt;

#[cfg(windows)]
fn exec_command(mut command: std::process::Command) {
    command.spawn().expect("failed to execute command")
        .wait().expect("failed to wait on child process");
}

#[cfg(not(windows))]
fn exec_command(mut command: std::process::Command) {
    command.exec();
}

fn main() -> Result<()> {
    let matches = cli::b2m().get_matches();
    let config = cli::Config::new(&matches)?;
    if config.check {
        check(&config);
        return Ok(());
    }
    let mut extractor = find_extractor(&config)?;
    if let Some(path) = config.cookie {
        extractor.load_cookie(path.as_ref())?;
    }
    let res = extractor.extract()?;
    if config.info {
        if config.json {
            todo!();
        } else {
            dbg!(res);
        }
    } else {
        exec_command(spwan_command(res, &config));
    }
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
    let mut extractors: Vec<Box<dyn Extractor>> = Vec::new();
    #[cfg(feature = "fina")]
    if matches!(conf.parser, Some("fina") | None) {
        match Fina::new(url) {
            Ok(ex) => extractors.push(Box::new(ex)),
            Err(e) => eprintln!("Error(finata): {}", e),
        }
    }
    // todo: check whether lux supports this url
    if matches!(conf.parser, Some("lux") | None) {
        extractors.push(Box::new(Lux::new(url)));
    }
    Ok(Box::new(extractors))
}
