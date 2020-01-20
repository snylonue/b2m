use serde_json::Value;
use super::search_displays;
use super::Url;

pub fn parse(value: &Value) -> Option<Url> {
    let displays = ["dash-flv", "dash-flv360", "dash-flv480", "dash-flv720", "flv", "flv360", "flv480", "flv720"];
    //json['streams'] is ordered with BTreeMap
    let (dp, stream) = search_displays(&value["streams"], &displays)?;
    if dp.matches("dash").next().is_none() {
        let video_url = stream["src"]
            .as_array()?
            .iter()
            .map(|x| { String::from(x.as_str().unwrap_or("")) })
            .collect();
        Some(Url::new(video_url, vec![]))
    } else {
        let dash_url = stream["src"].as_array()?;
        let video_url = vec![String::from(dash_url[0][0].as_str()?)];
        let audio_url = vec![String::from(dash_url[1][0].as_str()?)];
        Some(Url::new(video_url, audio_url))
    }
}