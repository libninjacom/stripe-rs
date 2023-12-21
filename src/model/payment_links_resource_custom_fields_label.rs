use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentLinksResourceCustomFieldsLabel {
    ///Custom text for the label, displayed to the customer. Up to 50 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<String>,
    ///The type of the label.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for PaymentLinksResourceCustomFieldsLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}