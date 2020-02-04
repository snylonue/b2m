use serde_json::Value;
use super::search_displays;
use super::Url;
use super::super::value_to_string;

const DISPLAYS: [&str; 6] = ["TD_H265", "TD", "HD_H265", "HD", "SD", "LD"];

pub fn parse_you_get(value: &Value) -> Option<Url> {
    let (_, stream) = search_displays(&value["streams"], &DISPLAYS)?;
    let video_url = stream["src"]
        .as_array()?
        .iter()
        .filter_map(|x| value_to_string!(x))
        .collect();
    Some(Url::new(video_url, Vec::new()))
}