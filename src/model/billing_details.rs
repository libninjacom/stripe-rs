use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BillingDetails {
    ///Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<serde_json::Value>,
    ///Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    ///Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///Billing phone number (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
impl std::fmt::Display for BillingDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}