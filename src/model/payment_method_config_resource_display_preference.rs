
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodConfigResourceDisplayPreference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overridable: Option<bool>,
    pub preference: String,
    pub value: String,
}
impl std::fmt::Display for PaymentMethodConfigResourceDisplayPreference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}