use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReceivedPaymentMethodDetailsFinancialAccount {
    ///The FinancialAccount ID.
    pub id: String,
    ///The rails the ReceivedCredit was sent over. A FinancialAccount can only send funds over `stripe`.
    pub network: String,
}
impl std::fmt::Display for ReceivedPaymentMethodDetailsFinancialAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}