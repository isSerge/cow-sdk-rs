use std::{fmt, str};

use alloy::primitives::FixedBytes;
use eyre::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OrderUid(pub FixedBytes<56>);

impl OrderUid {
    pub fn new(uid: FixedBytes<56>) -> Self {
        Self(uid)
    }
}

impl fmt::Display for OrderUid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl str::FromStr for OrderUid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(FixedBytes::from_str(s)?))
    }
}
