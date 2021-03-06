#[macro_export]
macro_rules! matched {
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
#[cfg(feature = "nfinata")]
pub mod netease_music;
pub mod youtube;

use crate::parsers::Url;
use crate::Config;
use crate::ResultInfo;
use serde_json::Value;

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
