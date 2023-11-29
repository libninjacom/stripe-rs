
use serde::{Serialize, Deserialize};
use super::Quote;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuotesResourceQuoteList {
    pub data: Vec<Quote>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for QuotesResourceQuoteList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}