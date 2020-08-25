use anyhow::Result;
use serde_json::Value;
use std::process;
use super::Parser;
use crate::command;
use crate::Config;

pub struct Annie;

impl Parser for Annie {
    fn run(url: &str, setting: &Config) -> Result<Value> {
        let mut cmd = process::Command::new("annie");
        if let Some(cookie) = &setting.cookie {
            cmd.arg("-c")
                .arg(cookie);
        }
        cmd.arg("-j")
            .arg(url)
            .stderr(process::Stdio::null());
        if let Some(proxy) = &setting.proxy {
           cmd.env("HTTP_PROXY", proxy.to_string());
        }
        let (stdout, _) = command::run_command(&mut cmd)?;
        let res: Value = parse_json!(&stdout);
        Ok(get!(res[0].clone(), res))
    }
    fn extract_infos(info: &Value) -> (Option<String>, Option<String>) {
        let referrer = value_to_string!(info["url"]);
        let title = value_to_string!(info["title"]);
        (referrer, title)
    }
}