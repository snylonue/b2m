use serde_json::Value;
use super::search_displays;
use super::Url;
use super::super::to_option_string;

const YOU_GET_DISPLAYS: [&str; 8] = ["dash-flv", "flv", "dash-flv720", "flv720", "dash-flv480", "flv480", "dash-flv360", "flv360"];
const ANNIE_DISPLAYS: [&str; 4] = ["80", "64", "32", "16"];

pub fn parse_you_get(value: &Value) -> Option<Url> {
    //json['streams'] is ordered with BTreeMap
    let (dp, stream) = search_displays(&value["streams"], &YOU_GET_DISPLAYS)?;
    if dp.matches("dash").next().is_none() {
        let video_url = stream["src"]
            .as_array()?
            .iter()
            .filter_map(|x| x.as_str().and_then(to_option_string))
            .collect();
        Some(Url::new(video_url, vec![]))
    } else {
        let dash_url = stream["src"].as_array()?;
        let video_url = vec![String::from(dash_url[0][0].as_str()?)];
        let audio_url = vec![String::from(dash_url[1][0].as_str()?)];
        Some(Url::new(video_url, audio_url))
    }
}
pub fn parse_annie(value: &Value) -> Option<Url> {
    let (_, stream) = search_displays(&value["streams"], &ANNIE_DISPLAYS)?;
    //println!("{:?}", stream);   
    let urls = stream["urls"]
        .as_array()?
        .iter()
        .filter_map(|x| x["url"].as_str().and_then(to_option_string))
        .collect();
    Some(Url::new(urls, Vec::new()))
}