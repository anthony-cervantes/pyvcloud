use std::fmt;

/// Constant representing one megabyte.
pub const SIZE_1MB: usize = 1024 * 1024;

/// Name of the system organization.
pub const SYSTEM_ORG_NAME: &str = "system";

/// Substring used for alpha API versions.
pub const ALPHA_API_SUBSTRING: &str = "alpha";

/// Basic set of credentials containing organisation, user and password.
#[derive(Debug, Clone)]
pub struct BasicLoginCredentials {
    pub user: String,
    pub org: String,
    pub password: String,
}

impl BasicLoginCredentials {
    /// Create a new credentials struct.
    pub fn new<U: Into<String>, O: Into<String>, P: Into<String>>(
        user: U,
        org: O,
        password: P,
    ) -> Self {
        Self {
            user: user.into(),
            org: org.into(),
            password: password.into(),
        }
    }
}

impl fmt::Display for BasicLoginCredentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}", self.user, self.org)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_obscures_password() {
        let creds = BasicLoginCredentials::new("user", "org", "secret");
        assert_eq!(creds.to_string(), "user@org");
    }
}
