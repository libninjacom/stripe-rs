
use serde::{Serialize, Deserialize};
use super::InvoiceItemThresholdReason;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvoiceThresholdReason {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_gte: Option<i64>,
    pub item_reasons: Vec<InvoiceItemThresholdReason>,
}
impl std::fmt::Display for InvoiceThresholdReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}