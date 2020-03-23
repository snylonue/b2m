use std::net::SocketAddr;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::fmt;
use std::convert::TryFrom;
use super::Res;

pub struct ProxyAddr<'a> {
    addr: SocketAddr,
    protocal: &'a str,
}

impl<'a> ProxyAddr<'a> {
    pub const fn new(addr: SocketAddr, protocal: &'a str) -> Self {
        Self { addr, protocal }
    }
    pub const fn from_addr(addr: SocketAddr) -> Self {
        Self { addr, protocal: "http" }
    }
    pub fn from_str(s: &'a str) -> Res<Self> {
        let mut splits = s.rsplit("://");
        match (splits.next(), splits.next()) {
            (Some(addr),Some(protocal)) => Ok(Self::new(addr.parse()?, protocal)),
            (Some(addr), None) => Ok(Self::from_addr(addr.parse()?)),
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
impl<'a> Default for ProxyAddr<'a> {
    fn default() -> Self {
        Self::from_addr(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 1080))
    }
}