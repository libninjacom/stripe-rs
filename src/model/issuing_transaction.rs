use serde::{Serialize, Deserialize};
use super::IssuingAuthorizationMerchantData;
/**Any use of an [issued card](https://stripe.com/docs/issuing) that results in funds entering or leaving
your Stripe account, such as a completed purchase or refund, is represented by an Issuing
`Transaction` object.

Related guide: [Issued card transactions](https://stripe.com/docs/issuing/purchases/transactions)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingTransaction {
    ///The transaction amount, which will be reflected in your balance. This amount is in your currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,
    ///Detailed breakdown of amount components. These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_details: Option<serde_json::Value>,
    ///The `Authorization` object that led to this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<serde_json::Value>,
    ///ID of the [balance transaction](https://stripe.com/docs/api/balance_transactions) associated with this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<serde_json::Value>,
    ///The card used to make this transaction.
    pub card: serde_json::Value,
    ///The cardholder to whom this transaction belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<serde_json::Value>,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///If you've disputed the transaction, the ID of the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute: Option<serde_json::Value>,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///The amount that the merchant will receive, denominated in `merchant_currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal). It will be different from `amount` if the merchant is taking payment in a different currency.
    pub merchant_amount: i64,
    ///The currency with which the merchant is taking payment.
    pub merchant_currency: String,
    ///
    pub merchant_data: IssuingAuthorizationMerchantData,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    ///Details about the transaction, such as processing dates, set by the card network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_data: Option<serde_json::Value>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///Additional purchase information that is optionally provided by the merchant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_details: Option<serde_json::Value>,
    ///[Token](https://stripe.com/docs/api/issuing/tokens/object) object used for this transaction. If a network token was not used for this transaction, this field will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<serde_json::Value>,
    ///[Treasury](https://stripe.com/docs/api/treasury) details related to this transaction if it was created on a [FinancialAccount](/docs/api/treasury/financial_accounts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<serde_json::Value>,
    ///The nature of the transaction.
    #[serde(rename = "type")]
    pub type_: String,
    ///The digital wallet used for this transaction. One of `apple_pay`, `google_pay`, or `samsung_pay`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
}
impl std::fmt::Display for IssuingTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}