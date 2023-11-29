
use serde::{Serialize, Deserialize};
use super::LineItem;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvoiceLinesList {
    pub data: Vec<LineItem>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for InvoiceLinesList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}