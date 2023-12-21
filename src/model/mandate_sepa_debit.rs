use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MandateSepaDebit {
    ///The unique reference of the mandate.
    pub reference: String,
    ///The URL of the mandate. This URL generally contains sensitive information about the customer and should be shared with them exclusively.
    pub url: String,
}
impl std::fmt::Display for MandateSepaDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}