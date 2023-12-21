use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisputePaymentMethodDetails {
    ///Card specific dispute details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<serde_json::Value>,
    ///Payment method type.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for DisputePaymentMethodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}