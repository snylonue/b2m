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

use serde_json::Value;
use super::proxy::ProxyAddr;
use super::parsers::Url;

type ResultInfo = super::Res<super::MediaInfo>;

pub trait Extractor {
    fn is_support(url: &str)  -> bool;
    fn real_url(value: &Value) -> Option<Url>;
    fn extract(url: &str, pxy: &Option<ProxyAddr>) -> ResultInfo;
}

pub fn search_by_keys<'a>(object: &'a Value, keys: &[&str]) -> Option<(&'a String, &'a Value)> {
    let object = object.as_object()?;
    for i in keys.iter() {
        match object.iter().find(|(x, _)| { x == i }) {
            Some(el) => return Some(el),
            None => continue,
        }
    }
    Some(object.iter().next()?)
}