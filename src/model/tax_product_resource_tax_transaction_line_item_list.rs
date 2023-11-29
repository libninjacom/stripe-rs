
use serde::{Serialize, Deserialize};
use super::TaxTransactionLineItem;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxProductResourceTaxTransactionLineItemList {
    pub data: Vec<TaxTransactionLineItem>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for TaxProductResourceTaxTransactionLineItemList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}