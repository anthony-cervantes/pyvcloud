use reqwest::header::InvalidHeaderValue;
use reqwest::StatusCode;
use thiserror::Error;
use url::ParseError as UrlParseError;

/// Errors returned by the vCloud client.
#[derive(Debug, Error)]
pub enum ClientError {
    /// Failed to parse a base URL.
    #[error("invalid base url: {0}")]
    InvalidBaseUrl(#[from] UrlParseError),

    /// Network or transport layer failure.
    #[error("request error: {0}")]
    Request(#[from] reqwest::Error),

    /// Invalid header value passed to a request.
    #[error("invalid header value: {0}")]
    InvalidHeader(#[from] InvalidHeaderValue),

    /// The server responded with a non-success status code.
    #[error("unexpected response status: {status}")]
    UnexpectedStatus { status: StatusCode },

    /// Authentication completed but no token header was returned.
    #[error("authentication did not return an authorization token")]
    MissingAuthorizationToken,

    /// JSON decoding failed.
    #[error("failed to parse response body: {0}")]
    Deserialize(#[from] serde_json::Error),
}
