use serde_json::Value;
use super::search_displays;
use super::Url;

pub fn parse(value: &Value) -> Option<Url> {
    let displays = ["TD_H265", "TD", "HD_H265", "HD", "SD", "LD"];
    let (_, stream) = search_displays(&value["streams"], &displays)?;
    let video_url = stream["src"]
        .as_array()?
        .iter()
        .map(|x| { String::from(x.as_str().unwrap_or("")) })
        .collect();
    Some(Url::new(video_url, vec![]))
}