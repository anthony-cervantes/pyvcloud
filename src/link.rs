#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Link {
    pub href: String,
    pub rel: String,
    pub media_type: Option<String>,
    pub name: Option<String>,
}

impl Link {
    pub fn new(href: impl Into<String>, rel: impl Into<String>) -> Self {
        Self {
            href: href.into(),
            rel: rel.into(),
            media_type: None,
            name: None,
        }
    }

    pub fn media_type(mut self, media_type: impl Into<String>) -> Self {
        self.media_type = Some(media_type.into());
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}
