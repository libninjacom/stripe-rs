use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerTax {
    ///Surfaces if automatic tax computation is possible given the current customer location information.
    pub automatic_tax: String,
    ///A recent IP address of the customer used for tax reporting and tax location inference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    ///The customer's location as identified by Stripe Tax.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<serde_json::Value>,
}
impl std::fmt::Display for CustomerTax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}