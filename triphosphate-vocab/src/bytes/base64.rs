use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};
use serde::{de, Deserializer, Serializer};

pub fn serialize<S>(b: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&STANDARD_NO_PAD.encode(b))
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
        type Value = Vec<u8>;

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            STANDARD_NO_PAD.decode(v).map_err(E::custom)
        }

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            std::write!(formatter, "a base64 encoded string")
        }
    }

    deserializer.deserialize_str(Visitor)
}
