
use serde::{Serialize, Deserialize};
use super::TaxProductResourceCustomerDetailsResourceTaxId;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxProductResourceCustomerDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    pub tax_ids: Vec<TaxProductResourceCustomerDetailsResourceTaxId>,
    pub taxability_override: String,
}
impl std::fmt::Display for TaxProductResourceCustomerDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}