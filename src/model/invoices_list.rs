
use serde::{Serialize, Deserialize};
use super::Invoice;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvoicesList {
    pub data: Vec<Invoice>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for InvoicesList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}