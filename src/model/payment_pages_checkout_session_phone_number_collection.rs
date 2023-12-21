use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentPagesCheckoutSessionPhoneNumberCollection {
    ///Indicates whether phone number collection is enabled for the session
    pub enabled: bool,
}
impl std::fmt::Display for PaymentPagesCheckoutSessionPhoneNumberCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}