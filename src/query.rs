use serde_json::Value;

use crate::{Client, Result};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SortDirection {
    Asc,
    Desc,
}

impl SortDirection {
    fn as_str(self) -> &'static str {
        match self {
            Self::Asc => "asc",
            Self::Desc => "desc",
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Query {
    entity_type: String,
    filter: Option<String>,
    fields: Vec<String>,
    sort: Option<(String, SortDirection)>,
    page: Option<u32>,
    page_size: Option<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryPage {
    pub body: String,
}

impl Query {
    pub fn new(entity_type: impl Into<String>) -> Self {
        Self {
            entity_type: entity_type.into(),
            ..Self::default()
        }
    }

    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    pub fn field(mut self, field: impl Into<String>) -> Self {
        self.fields.push(field.into());
        self
    }

    pub fn sort(mut self, field: impl Into<String>, direction: SortDirection) -> Self {
        self.sort = Some((field.into(), direction));
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn path(&self) -> String {
        let mut pairs = vec![format!("type={}", self.entity_type)];
        if let Some(filter) = &self.filter {
            pairs.push(format!("filter={filter}"));
        }
        if !self.fields.is_empty() {
            pairs.push(format!("fields={}", self.fields.join(",")));
        }
        if let Some((field, direction)) = &self.sort {
            pairs.push(format!("sort{}={field}", direction.as_str()));
        }
        if let Some(page) = self.page {
            pairs.push(format!("page={page}"));
        }
        if let Some(page_size) = self.page_size {
            pairs.push(format!("pageSize={page_size}"));
        }
        format!("/api/query?{}", pairs.join("&"))
    }

    pub fn execute_xml(&self, client: &Client) -> Result<QueryPage> {
        Ok(QueryPage {
            body: client.get(self.path()).send()?.body,
        })
    }

    pub fn execute_json(&self, client: &Client) -> Result<Value> {
        client
            .get(self.path())
            .accept("application/json")
            .send()?
            .json()
    }
}
