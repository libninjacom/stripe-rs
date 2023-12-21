use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentLinksResourceAutomaticTax {
    ///If `true`, tax will be calculated automatically using the customer's location.
    pub enabled: bool,
}
impl std::fmt::Display for PaymentLinksResourceAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}