use std::net::SocketAddr;
use std::fmt;
use std::convert::TryFrom;

pub struct ProxyAddr<'a> {
    addr: SocketAddr,
    protocal: &'a str,
}

impl<'a> ProxyAddr<'a> {
    pub fn new(addr: SocketAddr, protocal: &'a str) -> Self {
        Self { addr, protocal }
    }
    pub fn from_addr(addr: SocketAddr) -> Self {
        Self { addr, protocal: "http" }
    }
    pub fn from_str(s: &'a str) -> super::Res<Self> {
        let splits = {
            let mut splits = s.split("://");
            let (pt, addr) = (splits.next(), splits.next());
            if let Some(_) = addr {
                (pt, addr)
            } else {
                (None, pt)
            }
        };
        match splits {
            (Some(protocal), Some(addr)) => Ok(Self::new(addr.parse()?, protocal)),
            (None, Some(addr)) => Ok(Self::from_addr(addr.parse()?)),
            _ => Err(failure::err_msg("Invailed proxy address syntax"))
        }
    }
}
impl<'a> fmt::Display for ProxyAddr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}://{}", self.protocal, self.addr)
    }
}
impl<'a> TryFrom<&'a str> for ProxyAddr<'a> {
    type Error = failure::Error;

    #[inline]
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}