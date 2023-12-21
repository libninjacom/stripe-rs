use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvoiceLineItemPeriod {
    ///The end of the period, which must be greater than or equal to the start. This value is inclusive.
    pub end: i64,
    ///The start of the period. This value is inclusive.
    pub start: i64,
}
impl std::fmt::Display for InvoiceLineItemPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}