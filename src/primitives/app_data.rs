use std::{
    borrow::Cow,
    fmt,
    fmt::{Debug, Formatter},
    str::FromStr,
};

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
pub struct AppDataHash(pub [u8; 32]);

impl AppDataHash {
    pub fn is_zero(&self) -> bool {
        *self == Self::default()
    }
}

impl Debug for AppDataHash {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "0x{}", hex::encode(self.0))
    }
}

impl FromStr for AppDataHash {
    type Err = hex::FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = [0u8; 32];
        hex::decode_to_slice(s.strip_prefix("0x").unwrap_or(s), &mut bytes)?;
        Ok(Self(bytes))
    }
}

impl Serialize for AppDataHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut bytes = [0u8; 2 + 32 * 2];
        bytes[..2].copy_from_slice(b"0x");
        // Can only fail if the buffer size does not match but we know it is correct.
        hex::encode_to_slice(self.0, &mut bytes[2..]).unwrap();
        // Hex encoding is always valid utf8.
        let s = std::str::from_utf8(&bytes).unwrap();
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for AppDataHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Cow::<str>::deserialize(deserializer)?;
        let value = s.parse().map_err(|err| {
            de::Error::custom(format!("failed to decode {s:?} as hex appdata 32 bytes: {err}"))
        })?;
        Ok(value)
    }
}

impl PartialEq<[u8; 32]> for AppDataHash {
    fn eq(&self, other: &[u8; 32]) -> bool {
        self.0 == *other
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppData {
    pub full_app_data: FullAppData,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullAppData {
    pub version: String,
    pub metadata: String,
}
