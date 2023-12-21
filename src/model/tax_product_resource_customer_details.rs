use serde::{Serialize, Deserialize};
use super::TaxProductResourceCustomerDetailsResourceTaxId;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxProductResourceCustomerDetails {
    ///The customer's postal address (for example, home or business location).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<serde_json::Value>,
    ///The type of customer address provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_source: Option<String>,
    ///The customer's IP address (IPv4 or IPv6).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    ///The customer's tax IDs (for example, EU VAT numbers).
    pub tax_ids: Vec<TaxProductResourceCustomerDetailsResourceTaxId>,
    ///The taxability override used for taxation.
    pub taxability_override: String,
}
impl std::fmt::Display for TaxProductResourceCustomerDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}