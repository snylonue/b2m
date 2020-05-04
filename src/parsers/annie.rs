use anyhow::Result;
use serde_json::Value;
use std::process;
use super::Parser;
use crate::command;
use crate::Setting;

pub struct Annie;

impl Parser for Annie {
    fn run(url: &str, setting: &Setting) -> Result<Value> {
        let mut cmd = process::Command::new("annie");
        cmd.arg("-j")
            .arg(url)
            .stderr(process::Stdio::null());
        if let Some(proxy) = &setting.proxy_addr {
            cmd.env("HTTP_PROXY", proxy.to_string());
        }
        let (stdout, _) = command::run_command(&mut cmd)?;
        Ok(parse_json!(&stdout))
    }
    fn extract_infos(info: &Value) -> (Option<String>, Option<String>) {
        let referrer = value_to_string!(info["url"]);
        let title = value_to_string!(info["title"]);
        (referrer, title)
    }
}