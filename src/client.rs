use crate::error::ClientError;
use crate::types::{OrgListResponse, Organization};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION};
use reqwest::StatusCode;
use std::time::Duration;
use tracing::instrument;
use url::Url;

/// Blocking client for interacting with vCloud Director APIs.
#[derive(Debug)]
pub struct VCloudClient {
    base_url: Url,
    organization: String,
    http: Client,
    token: Option<String>,
}

impl VCloudClient {
    /// Builds a new client with sane defaults.
    pub fn new(
        base_url: impl AsRef<str>,
        organization: impl Into<String>,
    ) -> Result<Self, ClientError> {
        let base_url = Url::parse(base_url.as_ref())?;
        let http = Client::builder().timeout(Duration::from_secs(30)).build()?;

        Ok(Self {
            base_url,
            organization: organization.into(),
            http,
            token: None,
        })
    }

    /// Configure a bearer token for subsequent requests.
    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Performs a credential-based login and captures the authorization token.
    #[instrument(level = "info", skip(self, password))]
    pub fn login_with_credentials(
        &mut self,
        username: &str,
        password: &str,
    ) -> Result<String, ClientError> {
        let login_url = self.base_url.join("/api/sessions")?;
        let response = self
            .http
            .post(login_url)
            .basic_auth(username, Some(password))
            .header(ACCEPT, "application/json")
            .send()?;

        if !response.status().is_success() {
            return Err(ClientError::UnexpectedStatus {
                status: response.status(),
            });
        }

        let token = response
            .headers()
            .get("x-vcloud-authorization")
            .and_then(|header| header.to_str().ok())
            .ok_or(ClientError::MissingAuthorizationToken)?
            .to_string();

        self.token = Some(token.clone());
        Ok(token)
    }

    /// Lists all organizations visible to the authenticated user.
    #[instrument(level = "info", skip(self))]
    pub fn list_organizations(&self) -> Result<Vec<Organization>, ClientError> {
        let orgs_url = self.base_url.join("/api/orgs")?;
        let mut headers = HeaderMap::new();
        if let Some(token) = &self.token {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", token))?,
            );
            headers.insert("x-vcloud-authorization", HeaderValue::from_str(token)?);
        }

        let response = self
            .http
            .get(orgs_url)
            .headers(headers)
            .header(ACCEPT, "application/json")
            .send()?;

        match response.status() {
            StatusCode::OK => {
                let body = response.text()?;
                let parsed: OrgListResponse = serde_json::from_str(&body)?;
                Ok(parsed.organizations)
            }
            status => Err(ClientError::UnexpectedStatus { status }),
        }
    }

    /// Checks connectivity to the vCloud API root.
    #[instrument(level = "info", skip(self))]
    pub fn health_check(&self) -> Result<(), ClientError> {
        let url = self.base_url.join("/api")?;
        let response = self
            .http
            .get(url)
            .header(ACCEPT, "application/json")
            .send()?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(ClientError::UnexpectedStatus {
                status: response.status(),
            })
        }
    }

    /// Returns the configured organization.
    pub fn organization(&self) -> &str {
        &self.organization
    }

    /// Returns the current authorization token, if one has been set.
    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::Method::GET;
    use httpmock::{Method::POST, MockServer};

    #[test]
    fn performs_credential_login_and_reads_token() {
        let server = MockServer::start();
        let login_mock = server.mock(|when, then| {
            when.method(POST)
                .path("/api/sessions")
                .header("accept", "application/json");
            then.status(200)
                .header("x-vcloud-authorization", "abc123")
                .body("{}{}");
        });

        let mut client = VCloudClient::new(server.base_url(), "System").unwrap();
        let token = client.login_with_credentials("user", "pass").unwrap();

        assert_eq!(token, "abc123");
        assert_eq!(client.token(), Some("abc123"));
        login_mock.assert();
    }

    #[test]
    fn lists_organizations_using_token() {
        let server = MockServer::start();
        let orgs_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/api/orgs")
                .header("authorization", "Bearer abc123")
                .header("x-vcloud-authorization", "abc123")
                .header("accept", "application/json");
            then.status(200).body(
                "{\"organizations\": [ { \"name\": \"System\", \"id\": \"urn:vcloud:org:1\", \"description\": \"System org\" } ] }",
            );
        });

        let mut client = VCloudClient::new(server.base_url(), "System").unwrap();
        client = client.with_token("abc123");
        let orgs = client.list_organizations().unwrap();

        assert_eq!(orgs.len(), 1);
        assert_eq!(orgs[0].name, "System");
        orgs_mock.assert();
    }

    #[test]
    fn health_check_surfaces_status() {
        let server = MockServer::start();
        let health_mock = server.mock(|when, then| {
            when.method(GET).path("/api");
            then.status(200).body("{}{}");
        });

        let client = VCloudClient::new(server.base_url(), "System").unwrap();
        client.health_check().unwrap();
        health_mock.assert();
    }

    #[test]
    fn rejects_unexpected_status() {
        let server = MockServer::start();
        let login_mock = server.mock(|when, then| {
            when.method(POST).path("/api/sessions");
            then.status(401);
        });

        let mut client = VCloudClient::new(server.base_url(), "System").unwrap();
        let err = client.login_with_credentials("user", "pass").unwrap_err();

        match err {
            ClientError::UnexpectedStatus { status } => {
                assert_eq!(status, StatusCode::UNAUTHORIZED)
            }
            _ => panic!("unexpected error variant"),
        }

        login_mock.assert();
    }
}
