use std::{fmt, str::FromStr};

use crate::{Error, Result};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ApiVersion(String);

impl ApiVersion {
    pub const SUPPORTED: &'static [&'static str] = &[
        "29.0",
        "30.0",
        "31.0",
        "32.0",
        "33.0",
        "34.0",
        "35.0",
        "36.0",
        "37.0.0-alpha",
    ];

    pub fn new(version: impl Into<String>) -> Result<Self> {
        let version = version.into();
        if Self::SUPPORTED.contains(&version.as_str()) {
            Ok(Self(version))
        } else {
            Err(Error::UnsupportedApiVersion(version))
        }
    }

    pub fn latest() -> Self {
        Self("36.0".to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn is_alpha(&self) -> bool {
        self.0.contains("alpha")
    }
}

impl Default for ApiVersion {
    fn default() -> Self {
        Self::latest()
    }
}

impl fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for ApiVersion {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}
