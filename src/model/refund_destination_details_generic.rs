use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RefundDestinationDetailsGeneric {
    ///The reference assigned to the refund.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    ///Status of the reference on the refund. This can be `pending`, `available` or `unavailable`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_status: Option<String>,
}
impl std::fmt::Display for RefundDestinationDetailsGeneric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}