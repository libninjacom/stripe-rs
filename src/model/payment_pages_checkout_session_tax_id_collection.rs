use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentPagesCheckoutSessionTaxIdCollection {
    ///Indicates whether tax ID collection is enabled for the session
    pub enabled: bool,
}
impl std::fmt::Display for PaymentPagesCheckoutSessionTaxIdCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}