
use serde::{Serialize, Deserialize};
use super::{
    TaxProductResourceCustomerDetails, TaxProductResourceTaxTransactionLineItemList,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxTransaction {
    pub created: i64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    pub customer_details: TaxProductResourceCustomerDetails,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<TaxProductResourceTaxTransactionLineItemList>,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    pub object: String,
    pub reference: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversal: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<serde_json::Value>,
    pub tax_date: i64,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for TaxTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}