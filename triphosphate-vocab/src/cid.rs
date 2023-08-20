#[derive(Debug, Clone)]
pub struct Cid {
    repr: String,
    cid: cid::Cid,
}

impl super::StringFormat for Cid {
    fn as_str(&self) -> &str {
        &self.repr
    }

    type Error = cid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Error> {
        let cid = cid::Cid::try_from(s)?;
        Ok(Self {
            cid,
            repr: s.to_owned(),
        })
    }
}

impl Cid {
    pub fn version(&self) -> cid::Version {
        self.cid.version()
    }
}

#[cfg(test)]
mod tests {
    use crate::StringFormat;

    use super::*;

    // Takend from https://github.com/bluesky-social/atproto/blob/main/packages/lexicon/tests/general.test.ts

    #[test]
    fn valid() {
        for s in [
            "bafyreidfayvfuwqa7qlnopdjiqrxzs6blmoeu4rujcjtnci5beludirz2a",
            "bafyreifi5bqq7og5qxedc5xllono4vlpnfvl4pcbskymzcm5kjmbhgobmu",
        ] {
            match Cid::from_str(s) {
                Ok(cid) => assert_eq!(cid.as_str(), s),
                Err(e) => panic!("failed to parse {s:?}: {e}"),
            }
        }
    }

    #[test]
    fn invalid() {
        for s in ["https://github.com/", "abapsdofiuwrpoiasdfuaspdfoiu"] {
            match Cid::from_str(s) {
                Err(_) => {}
                Ok(cid) => panic!("cid {s:?} unexpectedly parsed to {cid:?}"),
            }
        }
    }
}
