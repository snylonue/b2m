use anyhow::{anyhow, Result};
use finata::website::{
    bilibili::{Bangumi, Video},
    netease_music::{PlayList, Song},
    pixiv::{Collection, Pixiv},
};

use crate::Extractor;

pub struct Fina {
    extractor: Box<dyn Extractor>,
}

impl Fina {
    pub fn new(url: &str) -> Result<Self> {
        if let Ok(extr) = Video::new(url) {
            return Ok(Self {
                extractor: Box::new(extr),
            });
        }
        if let Ok(extr) = Bangumi::new(url) {
            return Ok(Self {
                extractor: Box::new(extr),
            });
        }
        if let Ok(extr) = Song::new(url) {
            return Ok(Self {
                extractor: Box::new(extr),
            });
        }
        if let Ok(extr) = PlayList::new(url) {
            return Ok(Self {
                extractor: Box::new(extr),
            });
        }
        if let Ok(extr) = Pixiv::new(url) {
            if url.contains("pixiv") {
                return Ok(Self {
                    extractor: Box::new(extr),
                });
            }
        }
        if let Ok(extr) = Collection::new(url) {
            if url.contains("pixiv") {
                return Ok(Self {
                    extractor: Box::new(extr),
                });
            }
        }
        Err(anyhow!("unsupported url"))
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
