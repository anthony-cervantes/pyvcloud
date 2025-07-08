use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiVersion {
    V29,
    V30,
    V31,
    V32,
    V33,
    V34,
    V35,
    V36,
    V37Alpha,
}

impl ApiVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            ApiVersion::V29 => "29.0",
            ApiVersion::V30 => "30.0",
            ApiVersion::V31 => "31.0",
            ApiVersion::V32 => "32.0",
            ApiVersion::V33 => "33.0",
            ApiVersion::V34 => "34.0",
            ApiVersion::V35 => "35.0",
            ApiVersion::V36 => "36.0",
            ApiVersion::V37Alpha => "37.0.0-alpha",
        }
    }
}

impl fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for ApiVersion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "29.0" => Ok(ApiVersion::V29),
            "30.0" => Ok(ApiVersion::V30),
            "31.0" => Ok(ApiVersion::V31),
            "32.0" => Ok(ApiVersion::V32),
            "33.0" => Ok(ApiVersion::V33),
            "34.0" => Ok(ApiVersion::V34),
            "35.0" => Ok(ApiVersion::V35),
            "36.0" => Ok(ApiVersion::V36),
            "37.0.0-alpha" => Ok(ApiVersion::V37Alpha),
            _ => Err(()),
        }
    }
}

pub const API_CURRENT_VERSIONS: &[ApiVersion] = &[
    ApiVersion::V29,
    ApiVersion::V30,
    ApiVersion::V31,
    ApiVersion::V32,
    ApiVersion::V33,
    ApiVersion::V34,
    ApiVersion::V35,
    ApiVersion::V36,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_version() {
        let v: ApiVersion = "31.0".parse().unwrap();
        assert_eq!(v, ApiVersion::V31);
    }

    #[test]
    fn display_version() {
        assert_eq!(ApiVersion::V34.to_string(), "34.0");
    }

    #[test]
    fn api_current_versions_length() {
        assert_eq!(API_CURRENT_VERSIONS.len(), 8);
    }
}
