
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentLinksResourceCompletionBehaviorConfirmationPage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<String>,
}
impl std::fmt::Display for PaymentLinksResourceCompletionBehaviorConfirmationPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}