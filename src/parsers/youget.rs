use serde_json::Value;
use std::process;
use super::Parser;
use super::Res;
use super::super::cmd;

pub struct YouGet;

impl Parser for YouGet {
    fn run(url: &str) -> Res<Value> {
        let (stdout, _) = cmd::run_command(process::Command::new("you-get")
            .arg(url)
            .arg("--json")
            .stderr(process::Stdio::null())
        )?;
        Ok(parse_json!(&stdout))
    }
    fn extract_infos(info: &Value) -> (Option<String>, Option<String>) {
        // referrer = json['extra']['referer'] || json_output['url']
        let referrer = value_to_string!(info["extra"]["referer"], info["url"]);
        let title = value_to_string!(info["title"]);
        (referrer, title)
    }
}