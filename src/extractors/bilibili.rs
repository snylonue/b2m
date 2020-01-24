use serde_json::Value;
use super::search_displays;
use super::Url;
use super::super::to_option_string;

const DISPLAYS: [&str; 8] = ["dash-flv", "flv", "dash-flv720", "flv720", "dash-flv480", "flv480", "dash-flv360", "flv360"];

pub fn parse(value: &Value) -> Option<Url> {
    //json['streams'] is ordered with BTreeMap
    let (dp, stream) = search_displays(&value["streams"], &DISPLAYS)?;
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