use serde_json::Value;
use std::process;
use super::Parser;
use super::Res;
use super::super::cmd;

pub struct Annie;

impl Parser for Annie {
    fn run(url: &str) -> Res<Value> {
        let (stdout, _) = cmd::run_command(process::Command::new("annie")
            .arg("-j")
            .arg(url)
            .stderr(process::Stdio::null())
        )?;
        Ok(parse_json!(&stdout))
    }
    fn extract_infos(info: &Value) -> (Option<String>, Option<String>) {
        let referrer = value_to_string!(info["url"]);
        let title = value_to_string!(info["title"]);
        (referrer, title)
    }
}