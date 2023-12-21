use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentLinksResourceTaxIdCollection {
    ///Indicates whether tax ID collection is enabled for the session.
    pub enabled: bool,
}
impl std::fmt::Display for PaymentLinksResourceTaxIdCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}