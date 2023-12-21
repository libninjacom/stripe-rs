use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RefundDestinationDetailsCard {
    ///Value of the reference number assigned to the refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    ///Status of the reference number on the refund. This can be `pending`, `available` or `unavailable`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_status: Option<String>,
    ///Type of the reference number assigned to the refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<String>,
    ///The type of refund. This can be `refund`, `reversal`, or `pending`.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for RefundDestinationDetailsCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}