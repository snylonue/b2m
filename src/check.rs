use std::process;

use crate::command::parse_output;

const UNKNOWN: &str = "unknown";

pub fn check_mpv() -> bool {
    println!("Checking for mpv");
    println!("Running mpv -V");
    match process::Command::new("mpv").arg("-V").output() {
        Ok(r) => match parse_output(r) {
            Ok((stdout, stderr)) => {
                let version = stdout
                    .split(' ')
                    .nth(1)
                    .unwrap_or(UNKNOWN)
                    .trim_end_matches(',');
                println!("mpv version: {}\n", version);
                println!("Stdout:\n{}", stdout.trim_end());
                println!("Stderr:\n{}", stderr.trim_end());
                true
            }
            Err(e) => {
                eprintln!(
                    "Failed to check for mpv: unable to parse stdout and stderr:\n {:?}",
                    e
                );
                false
            }
        },
        Err(e) => {
            eprintln!("Failed to check for mpv: unable to run mpv:\n{:?}", e);
            false
        }
    }
}
