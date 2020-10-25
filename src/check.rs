use std::process;

use crate::command::parse_output;

const UNKNOWN: &str = "unknown";

pub fn check_you_get() -> bool {
    println!("Checking for you-get");
    if !cfg!(feature = "youget") {
        println!("Feature `youget` is not enabled, skip check");
        return false;
    }
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
                println!("Stdout:\n{}", stdout.trim_end());
                println!("Stderr:\n{}", stderr.trim_end());
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
    if !cfg!(feature = "annie") {
        println!("Feature `annie` is not enabled, skip check");
        return false;
    }
    println!("Running annie -v");
    match process::Command::new("annie")
        .arg("-v")
        .output() {
            Ok(r) => match parse_output(r) {
                Ok((stdout, stderr)) => {
                    let stdout = stdout.trim();
                    let version = stdout.split(' ').nth(2).unwrap_or(&UNKNOWN).trim_end_matches(',');
                    println!("annie version: {}\n", version);
                    println!("Stdout:\n{}", stdout.trim_end());
                    println!("Stderr:\n{}", stderr.trim_end());
                    true  
                },
                Err(e) => {
                    eprintln!("Failed to check for annie: unable to parse stdout and stderr:\n{:?}", e);
                    false
                },
            }
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
            Ok(r) => match parse_output(r) {
               Ok((stdout, stderr)) => {
                   let version = stdout.split(' ').nth(1).unwrap_or(&UNKNOWN).trim_end_matches(',');
                   println!("mpv version: {}\n", version);
                   println!("Stdout:\n{}", stdout.trim_end());
                   println!("Stderr:\n{}", stderr.trim_end());
                   true
               },
               Err(e) => {
                   eprintln!("Failed to check for mpv: unable to parse stdout and stderr:\n {:?}", e);
                   false
               },
            },
            Err(e) => {
                eprintln!("Failed to check for mpv: unable to run mpv:\n{:?}", e);
                false
            }
        }
}