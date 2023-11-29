
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SourceReceiverFlow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    pub amount_charged: i64,
    pub amount_received: i64,
    pub amount_returned: i64,
    pub refund_attributes_method: String,
    pub refund_attributes_status: String,
}
impl std::fmt::Display for SourceReceiverFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}