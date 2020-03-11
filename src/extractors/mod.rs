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
use super::proxy;
use super::parsers::Url;

type ResultInfo = super::Res<super::MediaInfo>;

pub trait Extractor {
    fn is_support(url: &str)  -> bool;
    fn real_url(value: &Value) -> Option<Url>;
    fn extract(url: &str, pxy: &Option<proxy::ProxyAddr>) -> ResultInfo;
}

pub fn search_displays<'a>(object: &'a Value, displays: &[&str]) -> Option<(&'a String, &'a Value)> {
    let object = object.as_object()?;
    let mut res = None;
    for i in displays.iter() {
        match object.iter().find(|(x, _)| { x == i }) {
            Some(el) => {
                res = Some(el);
                break;
            },
            None => continue,
        }
    }
    match res {
        Some(_) => res,
        None => Some(object.iter().next()?)
    }
}