use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentIntentNextActionKonbiniStores {
    ///FamilyMart instruction details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub familymart: Option<serde_json::Value>,
    ///Lawson instruction details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lawson: Option<serde_json::Value>,
    ///Ministop instruction details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ministop: Option<serde_json::Value>,
    ///Seicomart instruction details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seicomart: Option<serde_json::Value>,
}
impl std::fmt::Display for PaymentIntentNextActionKonbiniStores {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}