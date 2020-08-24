#[macro_export]
macro_rules! matched {
    ($url:expr, $reg: expr) => {
        {
            let reg = regex::Regex::new($reg).unwrap();
            reg.is_match($url)
        }
    };
    ($url:expr, $($regs: expr),*) => {
        {
            let regs = regex::RegexSet::new(&[
                $($regs,)*
            ]).unwrap();
            regs.is_match($url)
        }
    };
}

pub mod bilibili;
pub mod iqiyi;
pub mod youtube;
#[cfg(feature = "nfinata")]
pub mod netease_music;

use serde_json::Value;
use crate::Config;
use crate::parsers::Url;
use crate::ResultInfo;

pub trait Extractor {
    fn is_support(url: &str) -> bool;
    fn real_url(value: &Value) -> Option<Url>;
    fn extract(url: &str, setting: &Config) -> ResultInfo;
}

/// Searches an object with given keys in order, returns the first exist key and its value
pub fn search_by_keys<'a>(object: &'a Value, keys: &[&'a str]) -> Option<(&'a str, &'a Value)> {
    let object = object.as_object()?;
    for key in keys {
        match object.get(*key) {
            Some(v) => return Some((key.to_owned(), v)),
            None => continue,
        }
    }
    Some(object.iter().next().map(|(k, v)| (k.as_str(), v))?)
}