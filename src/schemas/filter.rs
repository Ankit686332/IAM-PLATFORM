use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub search: Option<String>,
}