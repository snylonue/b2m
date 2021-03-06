use super::Parser;
use crate::command;
use crate::Config;
use anyhow::Result;
use serde_json::Value;
use std::process;

pub struct YouGet;

impl Parser for YouGet {
    fn run(url: &str, setting: &Config) -> Result<Value> {
        let mut cmd = process::Command::new("you-get");
        cmd.arg(url).arg("--json");
        if let Some(proxy) = &setting.proxy {
            match proxy.protocal() {
                "http" => cmd.arg("-x"),
                "socks5" => cmd.arg("-s"),
                p => {
                    return Err(anyhow::anyhow!(
                        "protocal {} is not supported by you-get",
                        p
                    ))
                }
            };
            cmd.arg(proxy.to_string());
        }
        if let Some(cookie) = &setting.cookie {
            cmd.arg("-c").arg(cookie);
        }
        let (stdout, _) = command::run_command(&mut cmd)?;
        Ok(parse_json!(&stdout))
    }
    fn extract_infos(info: &Value) -> (Option<String>, Option<String>) {
        // referrer = json['extra']['referer'] || json_output['url']
        let referrer = value_to_string!(info["extra"]["referer"], info["url"]);
        let title = value_to_string!(info["title"]);
        (referrer, title)
    }
}
