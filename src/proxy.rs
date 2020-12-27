use std::fmt;
use std::str::FromStr;
use std::convert::Infallible;

#[derive(Debug, Clone)]
pub struct ProxyAddr {
    addr: String,
    protocal: String,
}

impl ProxyAddr {
    pub const fn new(addr: String, protocal: String) -> Self {
        Self { addr, protocal }
    }
    pub fn with_http(addr: String) -> Self {
        Self {
            addr,
            protocal: "http".to_owned(),
        }
    }
    pub fn protocal(&self) -> &str {
        &self.protocal
    }
    pub fn addr(&self) -> &str {
        &self.addr
    }
}
impl fmt::Display for ProxyAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}://{}", self.protocal, self.addr)
    }
}
impl FromStr for ProxyAddr {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.rsplit("://");
        match (splits.next(), splits.next()) {
            (Some(addr), Some(protocal)) => Ok(Self::new(addr.to_owned(), protocal.to_owned())),
            (Some(addr), None) => Ok(Self::with_http(addr.to_owned())),
            _ => unreachable!(),
        }
    }
}
impl Default for ProxyAddr {
    fn default() -> Self {
        Self::with_http("127.0.0.1:1080".to_owned())
    }
}
