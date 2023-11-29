
use serde::{Serialize, Deserialize};
use super::{
    CustomerAcceptance, MandateMultiUse, MandatePaymentMethodDetails, MandateSingleUse,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mandate {
    pub customer_acceptance: CustomerAcceptance,
    pub id: String,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_use: Option<MandateMultiUse>,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    pub payment_method: serde_json::Value,
    pub payment_method_details: MandatePaymentMethodDetails,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_use: Option<MandateSingleUse>,
    pub status: String,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for Mandate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}