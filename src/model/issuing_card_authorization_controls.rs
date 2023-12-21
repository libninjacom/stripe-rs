use serde::{Serialize, Deserialize};
use super::IssuingCardSpendingLimit;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingCardAuthorizationControls {
    ///Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations to allow. All other categories will be blocked. Cannot be set with `blocked_categories`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_categories: Option<Vec<String>>,
    ///Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations to decline. All other categories will be allowed. Cannot be set with `allowed_categories`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_categories: Option<Vec<String>>,
    ///Limit spending with amount-based rules that apply across any cards this card replaced (i.e., its `replacement_for` card and _that_ card's `replacement_for` card, up the chain).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limits: Option<Vec<IssuingCardSpendingLimit>>,
    ///Currency of the amounts within `spending_limits`. Always the same as the currency of the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limits_currency: Option<String>,
}
impl std::fmt::Display for IssuingCardAuthorizationControls {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}