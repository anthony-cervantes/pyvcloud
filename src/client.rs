use std::time::Duration;

use reqwest::{blocking, header, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::Value;
use url::Url;

use crate::{ApiVersion, Error, Result, Task, VcdError};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Auth {
    None,
    Bearer(String),
    Basic { username: String, password: String },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

impl From<Method> for reqwest::Method {
    fn from(value: Method) -> Self {
        match value {
            Method::Get => reqwest::Method::GET,
            Method::Post => reqwest::Method::POST,
            Method::Put => reqwest::Method::PUT,
            Method::Delete => reqwest::Method::DELETE,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Client {
    inner: blocking::Client,
    base_url: Url,
    api_version: ApiVersion,
    auth: Auth,
    user_agent: String,
}

#[derive(Clone, Debug)]
pub struct ClientBuilder {
    base_url: Url,
    api_version: ApiVersion,
    auth: Auth,
    timeout: Duration,
    accept_invalid_certs: bool,
    user_agent: String,
}

#[derive(Clone, Debug)]
pub struct RequestBuilder {
    client: Client,
    method: Method,
    path_or_url: String,
    accept: Option<String>,
    content_type: Option<String>,
    body: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResponseBody {
    pub status: StatusCode,
    pub request_id: Option<String>,
    pub content_type: Option<String>,
    pub access_token: Option<String>,
    pub body: String,
}

impl ClientBuilder {
    pub fn new(base_url: impl AsRef<str>) -> Result<Self> {
        Ok(Self {
            base_url: Url::parse(base_url.as_ref())?,
            api_version: ApiVersion::default(),
            auth: Auth::None,
            timeout: Duration::from_secs(120),
            accept_invalid_certs: false,
            user_agent: format!("pyvcloud-rust/{}", env!("CARGO_PKG_VERSION")),
        })
    }

    pub fn api_version(mut self, version: ApiVersion) -> Self {
        self.api_version = version;
        self
    }

    pub fn bearer_token(mut self, token: impl Into<String>) -> Self {
        self.auth = Auth::Bearer(token.into());
        self
    }

    pub fn basic_auth(mut self, username: impl Into<String>, password: impl Into<String>) -> Self {
        self.auth = Auth::Basic {
            username: username.into(),
            password: password.into(),
        };
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn accept_invalid_certs(mut self, accept_invalid_certs: bool) -> Self {
        self.accept_invalid_certs = accept_invalid_certs;
        self
    }

    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    pub fn build(self) -> Result<Client> {
        let inner = blocking::Client::builder()
            .timeout(self.timeout)
            .danger_accept_invalid_certs(self.accept_invalid_certs)
            .user_agent(self.user_agent.clone())
            .build()?;
        Ok(Client {
            inner,
            base_url: self.base_url,
            api_version: self.api_version,
            auth: self.auth,
            user_agent: self.user_agent,
        })
    }
}

impl Client {
    pub fn builder(base_url: impl AsRef<str>) -> Result<ClientBuilder> {
        ClientBuilder::new(base_url)
    }

    pub fn new(base_url: impl AsRef<str>) -> Result<Self> {
        Self::builder(base_url)?.build()
    }

    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    pub fn api_version(&self) -> &ApiVersion {
        &self.api_version
    }

    pub fn auth(&self) -> &Auth {
        &self.auth
    }

    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }

    pub fn with_bearer_token(mut self, token: impl Into<String>) -> Self {
        self.auth = Auth::Bearer(token.into());
        self
    }

    pub fn request(&self, method: Method, path_or_url: impl Into<String>) -> RequestBuilder {
        RequestBuilder {
            client: self.clone(),
            method,
            path_or_url: path_or_url.into(),
            accept: None,
            content_type: None,
            body: None,
        }
    }

    pub fn get(&self, path_or_url: impl Into<String>) -> RequestBuilder {
        self.request(Method::Get, path_or_url)
    }

    pub fn post(&self, path_or_url: impl Into<String>) -> RequestBuilder {
        self.request(Method::Post, path_or_url)
    }

    pub fn put(&self, path_or_url: impl Into<String>) -> RequestBuilder {
        self.request(Method::Put, path_or_url)
    }

    pub fn delete(&self, path_or_url: impl Into<String>) -> RequestBuilder {
        self.request(Method::Delete, path_or_url)
    }

    pub fn login_basic(
        mut self,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Result<Self> {
        let username = username.into();
        let password = password.into();
        let mut session_client = self.clone();
        session_client.auth = Auth::Basic { username, password };
        let response = session_client
            .post("/cloudapi/1.0.0/sessions/provider")
            .send()?;

        if let Some(token) = response.body_header_token() {
            self.auth = Auth::Bearer(token);
            Ok(self)
        } else {
            Err(Error::Missing("X-VMWARE-VCLOUD-ACCESS-TOKEN header"))
        }
    }

    fn url_for(&self, path_or_url: &str) -> Result<Url> {
        if path_or_url.starts_with("http://") || path_or_url.starts_with("https://") {
            Ok(Url::parse(path_or_url)?)
        } else {
            Ok(self.base_url.join(path_or_url.trim_start_matches('/'))?)
        }
    }
}

impl RequestBuilder {
    pub fn accept(mut self, value: impl Into<String>) -> Self {
        self.accept = Some(value.into());
        self
    }

    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    pub fn json(mut self, value: &Value) -> Result<Self> {
        self.content_type = Some("application/json".to_owned());
        self.accept = Some("application/json".to_owned());
        self.body = Some(serde_json::to_string(value)?);
        Ok(self)
    }

    pub fn xml(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(format!(
            "application/vnd.vmware.vcloud+xml;version={}",
            self.client.api_version
        ));
        self.accept = self.content_type.clone();
        self.body = Some(value.into());
        self
    }

    pub fn header(mut self, name: &str, value: impl Into<String>) -> Self {
        if name.eq_ignore_ascii_case(header::ACCEPT.as_str()) {
            self.accept = Some(value.into());
        } else if name.eq_ignore_ascii_case(header::CONTENT_TYPE.as_str()) {
            self.content_type = Some(value.into());
        }
        self
    }

    pub fn send(self) -> Result<ResponseBody> {
        let url = self.client.url_for(&self.path_or_url)?;
        let mut request = self.client.inner.request(self.method.into(), url).header(
            header::ACCEPT,
            self.accept.unwrap_or_else(|| {
                format!("application/*+xml;version={}", self.client.api_version)
            }),
        );

        request = match &self.client.auth {
            Auth::None => request,
            Auth::Bearer(token) => request.bearer_auth(token),
            Auth::Basic { username, password } => request.basic_auth(username, Some(password)),
        };

        if let Some(content_type) = self.content_type {
            request = request.header(header::CONTENT_TYPE, content_type);
        }
        if let Some(body) = self.body {
            request = request.body(body);
        }

        let response = request.send()?;
        let status = response.status();
        let request_id = response
            .headers()
            .get("X-VMWARE-VCLOUD-REQUEST-ID")
            .and_then(|value| value.to_str().ok())
            .map(ToOwned::to_owned);
        let content_type = response
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .map(ToOwned::to_owned);
        let access_token = response
            .headers()
            .get("X-VMWARE-VCLOUD-ACCESS-TOKEN")
            .and_then(|value| value.to_str().ok())
            .map(ToOwned::to_owned);
        let body = response.text()?;
        let response = ResponseBody {
            status,
            request_id,
            content_type,
            access_token,
            body,
        };

        if response.status.is_success() {
            Ok(response)
        } else {
            Err(Error::Response {
                status,
                error: Box::new(response.to_vcd_error()),
            })
        }
    }
}

impl ResponseBody {
    pub fn json<T: DeserializeOwned>(&self) -> Result<T> {
        Ok(serde_json::from_str(&self.body)?)
    }

    pub fn task(&self) -> Result<Task> {
        Task::from_xml(&self.body)
    }

    fn body_header_token(&self) -> Option<String> {
        self.access_token.clone()
    }

    fn to_vcd_error(&self) -> VcdError {
        VcdError {
            status_code: self.status.as_u16(),
            request_id: self.request_id.clone(),
            major_error_code: extract_xml_attr(&self.body, "majorErrorCode"),
            minor_error_code: extract_xml_attr(&self.body, "minorErrorCode"),
            message: extract_xml_attr(&self.body, "message").or_else(|| Some(self.body.clone())),
            body: self.body.clone(),
        }
    }
}

fn extract_xml_attr(xml: &str, name: &str) -> Option<String> {
    let pattern = format!("{name}=\"");
    let start = xml.find(&pattern)? + pattern.len();
    let end = xml[start..].find('"')?;
    Some(xml[start..start + end].to_owned())
}
