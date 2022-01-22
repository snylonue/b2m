macro_rules! parse_json {
    ($s: expr) => {
        match serde_json::from_str($s) {
            Ok(v) => v,
            Err(_) => return Err(anyhow::anyhow!(format!("Invalid json data: {}", $s))),
        }
    };
}
macro_rules! get {
    ($v: expr) => {
        $v
    };
    ($v: expr, $($vn: expr),+) => {
        match $v {
            serde_json::Value::Null => get!($($vn),+),
            _ => $v,
        }
    }
}

pub mod lux;
// pub mod youget;
