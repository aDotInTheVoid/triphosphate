use serde::{Deserialize, Serialize};

use crate::StringFormat;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CidLink {
    #[serde(rename = "$link")]
    link: crate::Cid,
}

impl CidLink {
    pub fn from_str(s: &str) -> Result<Self, cid::Error> {
        Ok(Self {
            link: crate::Cid::from_str(s)?,
        })
    }
}
