use anyhow::Result;

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
    fn extract(&mut self) -> anyhow::Result<finata::Finata> {
        self.extractor.extract()
    }

    fn load_netscape_cookie(&mut self, cookie: &std::path::Path) -> anyhow::Result<()> {
        self.extractor.load_netscape_cookie(cookie)
    }
}
