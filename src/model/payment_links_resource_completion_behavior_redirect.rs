use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentLinksResourceCompletionBehaviorRedirect {
    ///The URL the customer will be redirected to after the purchase is complete.
    pub url: String,
}
impl std::fmt::Display for PaymentLinksResourceCompletionBehaviorRedirect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}