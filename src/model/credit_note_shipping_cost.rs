use serde::{Serialize, Deserialize};
///When shipping_cost contains the shipping_rate from the invoice, the shipping_cost is included in the credit note.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditNoteShippingCost {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<String>,
}
impl std::fmt::Display for CreditNoteShippingCost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}