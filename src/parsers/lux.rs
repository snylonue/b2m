use crate::command;
use crate::Extractor;
use anyhow::Result;
use anyhow::anyhow;
use finata::Finata;
use finata::Origin;
use finata::Track;
use serde_json::Value;
use std::path::Path;
use std::path::PathBuf;
use std::process;

#[derive(Debug, Default)]
pub struct Lux {
    url: String,
    cookie: Option<PathBuf>,
    proxy: Option<String>,
}

impl Lux {
    pub fn new(url: &str) -> Self {
        Self { url: url.to_owned(), ..Default::default() }
    }
    pub fn run(&self) -> Result<Value> {
        let mut cmd = process::Command::new("lux");
        if let Some(cookie) = &self.cookie {
            cmd.arg("-c").arg(cookie);
        }
        if let Some(proxy) = &self.proxy {
            cmd.env("HTTP_PROXY", proxy);
        }
        cmd.arg("-j").arg(&self.url);
        let (stdout, _) = command::run_command(&mut cmd)?;
        let res: Value = parse_json!(&stdout);
        Ok(get!(res[0].clone(), res))
    }
    pub fn load_proxy(&mut self, proxy: &str) -> Result<()> {
        self.proxy = Some(proxy.to_owned());
        Ok(())
    }
}

impl Extractor for Lux {
    fn extract(&mut self) -> Result<finata::Finata> {
        let res = dbg!(self.run()?);
        let stream = match search_highest_quality(&res["streams"]) {
            Some(s) => s,
            None => return Err(anyhow!("fails to find stream")),
        };
        let mut tracks = Vec::with_capacity(stream.len());
        for part in stream {
            // todo: remove unwrap()
            let url = part["url"].as_str().unwrap().parse()?;
            // todo: use a better way to distinguish videos and audios
            match part["ext"].as_str() {
                Some("mp4") => tracks.push(Track::Video(url)),
                Some("m4a") => tracks.push(Track::Audio(url)),
                _ => tracks.push(Track::Video(url)),
            }
        }
        let title = res["url"].as_str();
        let origin = Origin::new(tracks, String::new());
        Ok(Finata::new(vec![origin], title.unwrap_or_default().to_owned()))
    }
    fn load_netscape_cookie(&mut self, cookie: &Path) -> Result<()> {
        self.cookie = Some(cookie.to_owned());
        Ok(())
    }
}

fn search_highest_quality<'a>(object: &'a Value) -> Option<&'a Vec<Value>> {
    let object = object.as_object()?;
    dbg!(object.iter().max_by_key(|(_, v)| v["size"].as_u64()).map(|(_, v)| v))?["parts"].as_array()
}