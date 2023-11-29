
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReceivedPaymentMethodDetailsFinancialAccount {
    pub id: String,
    pub network: String,
}
impl std::fmt::Display for ReceivedPaymentMethodDetailsFinancialAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}