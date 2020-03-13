use std::process;

use crate::command::parse_output;

const UNKNOWN: &str = "unknown";

pub fn check_you_get() -> bool {
    println!("Checking for you-get");
    println!("Running you-get -V");
    match process::Command::new("you-get")
        .arg("-V")
        .output() {
            Ok(r) => {
                let (stdout, stderr) = match parse_output(r) {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("Failed to check for you-get: unable to parse stdout and stderr:\n{:?}", e);
                        return false;
                    },
                };
                let splits = stderr.split(' ').collect::<Vec<_>>();
                let version = splits.get(2).unwrap_or(&UNKNOWN);
                println!("you-get version: {}\n", version);
                println!("{}", format!("Stdout:\n{}", stdout).trim());
                println!("{}", format!("Stderr:\n{}", stderr).trim());
                true
            },
            Err(e) => {
                eprintln!("Failed to check for you-get: unable to run you-get:\n{:?}", e);
                false
            }
        }
}
pub fn check_annie() -> bool {
    println!("Checking for annie");
    println!("Running annie -v");
    match process::Command::new("annie")
        .arg("-v")
        .output() {
            Ok(r) => {
                let (stdout, stderr) = match parse_output(r) {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("Failed to check for annie: unable to parse stdout and stderr:\n{:?}", e);
                        return false;
                    },
                };
                let stdout = stdout.trim();
                let splits = stdout.split(' ').collect::<Vec<_>>();
                let version = splits.get(2).unwrap_or(&UNKNOWN).trim_end_matches(',');
                println!("annie version: {}\n", version);
                println!("{}", format!("Stdout:\n{}", stdout).trim());
                println!("{}", format!("Stderr:\n{}", stderr).trim());
                true
            },
            Err(e) => {
                eprintln!("Failed to check for annie: unable to run annie:\n{:?}", e);
                false
            }
        }
}
pub fn check_mpv() -> bool {
    println!("Checking for mpv");
    println!("Running mpv -V");
    match process::Command::new("mpv")
        .arg("-V")
        .output() {
            Ok(r) => {
                let (stdout, stderr) = match parse_output(r) {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("Failed to check for mpv: unable to parse stdout and stderr:\n {:?}", e);
                        return false;
                    },
                };
                let splits = stdout.split(' ').collect::<Vec<_>>();
                let version = splits.get(1).unwrap_or(&UNKNOWN).trim_end_matches(',');
                println!("mpv version: {}\n", version);
                println!("{}", format!("Stdout:\n{}", stdout).trim());
                println!("{}", format!("Stderr:\n{}", stderr).trim());
                true
            },
            Err(e) => {
                eprintln!("Failed to check for mpv: unable to run mpv:\n{:?}", e);
                false
            }
        }
}