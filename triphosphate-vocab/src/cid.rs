use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Cid {
    repr: String, // TODO: Is this needed?
    pub(crate) cid: cid::Cid,
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

    pub(crate) fn from_cid(cid: cid::Cid) -> Self {
        let repr = cid.to_string();

        Self { cid, repr }
    }
}

impl fmt::Debug for Cid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cid({:?})", self.repr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Takend from https://github.com/bluesky-social/atproto/blob/main/packages/lexicon/tests/general.test.ts

    #[test]
    fn valid() {
        crate::tests::valids::<Cid>(&[
            "bafyreidfayvfuwqa7qlnopdjiqrxzs6blmoeu4rujcjtnci5beludirz2a",
            "bafyreifi5bqq7og5qxedc5xllono4vlpnfvl4pcbskymzcm5kjmbhgobmu",
        ]);
    }

    #[test]
    fn invalid() {
        crate::tests::invalids::<Cid>(&["https://github.com/", "abapsdofiuwrpoiasdfuaspdfoiu"]);
    }
}
