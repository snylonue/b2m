use serde_json::Value;

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
