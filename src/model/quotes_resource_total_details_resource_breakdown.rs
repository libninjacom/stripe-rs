
use serde::{Serialize, Deserialize};
use super::{LineItemsDiscountAmount, LineItemsTaxAmount};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuotesResourceTotalDetailsResourceBreakdown {
    pub discounts: Vec<LineItemsDiscountAmount>,
    pub taxes: Vec<LineItemsTaxAmount>,
}
impl std::fmt::Display for QuotesResourceTotalDetailsResourceBreakdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}