use serde::{Deserialize, Serialize};

/// Representation of an organization within vCloud Director.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Organization {
    /// Friendly organization name.
    pub name: String,
    /// Unique identifier exposed by the API.
    #[serde(rename = "id")]
    pub id: String,
    /// Optional description.
    #[serde(default)]
    pub description: String,
}

impl Organization {
    /// Creates a new organization instance.
    pub fn new(
        name: impl Into<String>,
        id: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            id: id.into(),
            description: description.into(),
        }
    }
}

/// Envelope returned by the organizations endpoint.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct OrgListResponse {
    pub organizations: Vec<Organization>,
}
