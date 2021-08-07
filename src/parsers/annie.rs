use crate::command;
use anyhow::Result;
use serde_json::Value;
use std::path::PathBuf;
use std::process;

#[derive(Debug, Default)]
pub struct Annie {
    cookie: Option<PathBuf>,
    proxy: Option<String>,
}

// impl Parser for Annie {
// fn run(url: &str, setting: &Config) -> Result<Value> {
// let mut cmd = process::Command::new("annie");
// if let Some(cookie) = &setting.cookie {
// cmd.arg("-c").arg(cookie);
// }
// cmd.arg("-j").arg(url);
// if let Some(proxy) = &setting.proxy {
// cmd.env("HTTP_PROXY", proxy.to_string());
// }
// let (stdout, _) = command::run_command(&mut cmd)?;
// let res: Value = parse_json!(&stdout);
// Ok(get!(res[0].clone(), res))
// }
// fn extract_infos(info: &Value) -> (Option<String>, Option<String>) {
// let referrer = value_to_string!(info["url"]);
// let title = value_to_string!(info["title"]);
// (referrer, title)
// }
// }

impl Annie {
    pub fn run(&self, url: &str) -> Result<Value> {
        let mut cmd = process::Command::new("annie");
        if let Some(cookie) = &self.cookie {
            cmd.arg("-c").arg(cookie);
        }
        if let Some(proxy) = &self.proxy {
            cmd.env("HTTP_PROXY", proxy);
        }
        cmd.arg("-j").arg(url);
        let (stdout, _) = command::run_command(&mut cmd)?;
        let res: Value = parse_json!(&stdout);
        Ok(get!(res[0].clone(), res))
    }
    pub fn load_netscape_cookie(&mut self, cookie: impl AsRef<std::path::Path>) -> Result<()> {
        self.cookie = Some(cookie.as_ref().to_owned());
        Ok(())
    }
    pub fn load_proxy(&mut self, proxy: &str) -> Result<()> {
        self.proxy = Some(proxy.to_owned());
        Ok(())
    }
}
