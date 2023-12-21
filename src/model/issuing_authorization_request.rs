use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingAuthorizationRequest {
    ///The `pending_request.amount` at the time of the request, presented in your card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal). Stripe held this amount from your account to fund the authorization if the request was approved.
    pub amount: i64,
    ///Detailed breakdown of amount components. These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_details: Option<serde_json::Value>,
    ///Whether this request was approved.
    pub approved: bool,
    ///A code created by Stripe which is shared with the merchant to validate the authorization. This field will be populated if the authorization message was approved. The code typically starts with the letter "S", followed by a six-digit number. For example, "S498162". Please note that the code is not guaranteed to be unique across authorizations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<String>,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///The `pending_request.merchant_amount` at the time of the request, presented in the `merchant_currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub merchant_amount: i64,
    ///The currency that was collected by the merchant and presented to the cardholder for the authorization. Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub merchant_currency: String,
    ///The card network's estimate of the likelihood that an authorization is fraudulent. Takes on values between 1 and 99.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_risk_score: Option<i64>,
    ///When an authorization is approved or declined by you or by Stripe, this field provides additional detail on the reason for the outcome.
    pub reason: String,
    ///If the `request_history.reason` is `webhook_error` because the direct webhook response is invalid (for example, parsing errors or missing parameters), we surface a more detailed error message via this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_message: Option<String>,
    ///Time when the card network received an authorization request from the acquirer in UTC. Referred to by networks as transmission time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_at: Option<i64>,
}
impl std::fmt::Display for IssuingAuthorizationRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}