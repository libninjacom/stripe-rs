use serde::{Serialize, Deserialize};
use super::{
    BalanceTransaction, IssuingAuthorizationMerchantData, IssuingAuthorizationRequest,
    IssuingAuthorizationVerificationData, IssuingCard, IssuingTransaction,
};
/**When an [issued card](https://stripe.com/docs/issuing) is used to make a purchase, an Issuing `Authorization`
object is created. [Authorizations](https://stripe.com/docs/issuing/purchases/authorizations) must be approved for the
purchase to be completed successfully.

Related guide: [Issued card authorizations](https://stripe.com/docs/issuing/purchases/authorizations)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingAuthorization {
    ///The total amount that was authorized or rejected. This amount is in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal). `amount` should be the same as `merchant_amount`, unless `currency` and `merchant_currency` are different.
    pub amount: i64,
    ///Detailed breakdown of amount components. These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_details: Option<serde_json::Value>,
    ///Whether the authorization has been approved.
    pub approved: bool,
    ///How the card details were provided.
    pub authorization_method: String,
    ///List of balance transactions associated with this authorization.
    pub balance_transactions: Vec<BalanceTransaction>,
    ///You can [create physical or virtual cards](https://stripe.com/docs/issuing/cards) that are issued to cardholders.
    pub card: IssuingCard,
    ///The cardholder to whom this authorization belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<serde_json::Value>,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///The currency of the cardholder. This currency can be different from the currency presented at authorization and the `merchant_currency` field on this authorization. Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///The total amount that was authorized or rejected. This amount is in the `merchant_currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal). `merchant_amount` should be the same as `amount`, unless `merchant_currency` and `currency` are different.
    pub merchant_amount: i64,
    ///The local currency that was presented to the cardholder for the authorization. This currency can be different from the cardholder currency and the `currency` field on this authorization. Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub merchant_currency: String,
    ///
    pub merchant_data: IssuingAuthorizationMerchantData,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    ///Details about the authorization, such as identifiers, set by the card network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_data: Option<serde_json::Value>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The pending authorization request. This field will only be non-null during an `issuing_authorization.request` webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_request: Option<serde_json::Value>,
    ///History of every time a `pending_request` authorization was approved/declined, either by you directly or by Stripe (e.g. based on your spending_controls). If the merchant changes the authorization by performing an incremental authorization, you can look at this field to see the previous requests for the authorization. This field can be helpful in determining why a given authorization was approved/declined.
    pub request_history: Vec<IssuingAuthorizationRequest>,
    ///The current status of the authorization in its lifecycle.
    pub status: String,
    ///[Token](https://stripe.com/docs/api/issuing/tokens/object) object used for this authorization. If a network token was not used for this authorization, this field will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<serde_json::Value>,
    ///List of [transactions](https://stripe.com/docs/api/issuing/transactions) associated with this authorization.
    pub transactions: Vec<IssuingTransaction>,
    ///[Treasury](https://stripe.com/docs/api/treasury) details related to this authorization if it was created on a [FinancialAccount](https://stripe.com/docs/api/treasury/financial_accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<serde_json::Value>,
    ///
    pub verification_data: IssuingAuthorizationVerificationData,
    ///The digital wallet used for this transaction. One of `apple_pay`, `google_pay`, or `samsung_pay`. Will populate as `null` when no digital wallet was utilized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
}
impl std::fmt::Display for IssuingAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}