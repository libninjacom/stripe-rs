use serde::{Serialize, Deserialize};
use super::{
    PaymentLinksResourceCompletionBehaviorConfirmationPage,
    PaymentLinksResourceCompletionBehaviorRedirect,
};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentLinksResourceAfterCompletion {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_confirmation: Option<
        PaymentLinksResourceCompletionBehaviorConfirmationPage,
    >,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<PaymentLinksResourceCompletionBehaviorRedirect>,
    ///The specified behavior after the purchase is complete.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for PaymentLinksResourceAfterCompletion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}