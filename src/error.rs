use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VcdError {
    pub status_code: u16,
    pub request_id: Option<String>,
    pub major_error_code: Option<String>,
    pub minor_error_code: Option<String>,
    pub message: Option<String>,
    pub body: String,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("unsupported VMware Cloud Director API version: {0}")]
    UnsupportedApiVersion(String),
    #[error("invalid URL: {0}")]
    Url(#[from] url::ParseError),
    #[error("HTTP client error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("XML error: {0}")]
    Xml(#[from] quick_xml::Error),
    #[error("VMware Cloud Director returned {status}: {error:?}")]
    Response {
        status: StatusCode,
        error: Box<VcdError>,
    },
    #[error("missing required value: {0}")]
    Missing(&'static str),
    #[error("task failed: {0}")]
    TaskFailed(String),
}
