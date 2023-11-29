
use serde::{Serialize, Deserialize};
use super::{QuotesResourceListLineItems, QuotesResourceTotalDetails};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotesResourceUpfront {
    pub amount_subtotal: i64,
    pub amount_total: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<QuotesResourceListLineItems>,
    pub total_details: QuotesResourceTotalDetails,
}
impl std::fmt::Display for QuotesResourceUpfront {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}