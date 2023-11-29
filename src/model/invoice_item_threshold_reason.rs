
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvoiceItemThresholdReason {
    pub line_item_ids: Vec<String>,
    pub usage_gte: i64,
}
impl std::fmt::Display for InvoiceItemThresholdReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}