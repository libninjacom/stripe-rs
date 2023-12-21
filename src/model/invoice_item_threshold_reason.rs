use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvoiceItemThresholdReason {
    ///The IDs of the line items that triggered the threshold invoice.
    pub line_item_ids: Vec<String>,
    ///The quantity threshold boundary that applied to the given line item.
    pub usage_gte: i64,
}
impl std::fmt::Display for InvoiceItemThresholdReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}