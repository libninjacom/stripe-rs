use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentLinksResourcePhoneNumberCollection {
    ///If `true`, a phone number will be collected during checkout.
    pub enabled: bool,
}
impl std::fmt::Display for PaymentLinksResourcePhoneNumberCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}