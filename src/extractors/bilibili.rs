use serde_json::Value;
use super::search_displays;
use super::Url;
use super::super::value_to_string;

const YOU_GET_DISPLAYS: [&str; 8] = ["dash-flv", "flv", "dash-flv720", "flv720", "dash-flv480", "flv480", "dash-flv360", "flv360"];
const ANNIE_DISPLAYS: [&str; 4] = ["80", "64", "32", "16"];

pub fn parse_you_get(value: &Value) -> Option<Url> {
    //json['streams'] is ordered with BTreeMap
    let (dp, stream) = search_displays(&value["streams"], &YOU_GET_DISPLAYS)?;
    if dp.matches("dash").next().is_some() {
        let dash_url = stream["src"].as_array()?;
        let video_url = vec![value_to_string!(dash_url[0][0])?];
        let audio_url = vec![value_to_string!(dash_url[1][0])?];
        Some(Url::new(video_url, audio_url))
    } else {
        let video_url = stream["src"]
            .as_array()?
            .iter()
            .filter_map(|x| value_to_string!(x))
            .collect();
        Some(Url::new(video_url, Vec::new()))
    }
}
pub fn parse_annie(value: &Value) -> Option<Url> {
    let (_, stream) = search_displays(&value["streams"], &ANNIE_DISPLAYS)?;
    let urls = stream["urls"]
        .as_array()?
        .iter()
        .filter_map(|x| value_to_string!(x["url"]))
        .collect();
    Some(Url::new(urls, Vec::new()))
}