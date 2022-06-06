use anyhow::Result;
use finata::{ExtractSync, Finata};
use reqwest::Proxy;
use std::path::Path;

use crate::Extractor;

pub struct Fina {
    extractor: Box<dyn finata::website::Extractor>,
}

impl Fina {
    pub fn new(url: &str) -> Result<Self> {
        Ok(Self {
            extractor: finata::website::choose_extractor(url)?,
        })
    }
}

impl Extractor for Fina {
    fn name(&self) -> &'static str {
        "finata"
    }

    fn extract(&mut self) -> Result<Finata> {
        Ok(self.extractor.extract_sync()?)
    }

    fn load_cookie(&mut self, cookie: &Path) -> Result<()> {
        Ok(self.extractor.client_mut().load_netscape_cookie(cookie)?)
    }

    fn set_proxy(&mut self, proxy: crate::proxy::ProxyAddr) -> Result<()> {
        *self.extractor.client_mut().client_mut() = reqwest::ClientBuilder::new().proxy(Proxy::all(&proxy.to_string())?).build()?;
        Ok(())
    }
}
