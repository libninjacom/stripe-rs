use serde::{Serialize, Deserialize};
use super::{QuotesResourceListLineItems, QuotesResourceTotalDetails};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuotesResourceUpfront {
    ///Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,
    ///Total after discounts and taxes are applied.
    pub amount_total: i64,
    ///The line items that will appear on the next invoice after this quote is accepted. This does not include pending invoice items that exist on the customer but may still be included in the next invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<QuotesResourceListLineItems>,
    ///
    pub total_details: QuotesResourceTotalDetails,
}
impl std::fmt::Display for QuotesResourceUpfront {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}