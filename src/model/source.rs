use serde::{Serialize, Deserialize};
use super::{
    SourceCodeVerificationFlow, SourceOrder, SourceReceiverFlow, SourceRedirectFlow,
    SourceTypeAchCreditTransfer, SourceTypeAchDebit, SourceTypeAcssDebit,
    SourceTypeAlipay, SourceTypeAuBecsDebit, SourceTypeBancontact, SourceTypeCard,
    SourceTypeCardPresent, SourceTypeEps, SourceTypeGiropay, SourceTypeIdeal,
    SourceTypeKlarna, SourceTypeMultibanco, SourceTypeP24, SourceTypeSepaDebit,
    SourceTypeSofort, SourceTypeThreeDSecure, SourceTypeWechat,
};
/**`Source` objects allow you to accept a variety of payment methods. They
represent a customer's payment instrument, and can be used with the Stripe API
just like a `Card` object: once chargeable, they can be charged, or can be
attached to customers.

Stripe doesn't recommend using the deprecated [Sources API](https://stripe.com/docs/api/sources).
We recommend that you adopt the [PaymentMethods API](https://stripe.com/docs/api/payment_methods).
This newer API provides access to our latest features and payment method types.

Related guides: [Sources API](https://stripe.com/docs/sources) and [Sources & Customers](https://stripe.com/docs/sources/customers).*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Source {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_credit_transfer: Option<SourceTypeAchCreditTransfer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_debit: Option<SourceTypeAchDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<SourceTypeAcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<SourceTypeAlipay>,
    ///A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the total amount associated with the source. This is the amount for which the source will be chargeable once ready. Required for `single_use` sources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<SourceTypeAuBecsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<SourceTypeBancontact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<SourceTypeCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<SourceTypeCardPresent>,
    ///The client secret of the source. Used for client-side retrieval using a publishable key.
    pub client_secret: String,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_verification: Option<SourceCodeVerificationFlow>,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) associated with the source. This is the currency for which the source will be chargeable once ready. Required for `single_use` sources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    ///The ID of the customer to which this source is attached. This will not be present when the source has not been attached to a customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<SourceTypeEps>,
    ///The authentication `flow` of the source. `flow` is one of `redirect`, `receiver`, `code_verification`, `none`.
    pub flow: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<SourceTypeGiropay>,
    ///Unique identifier for the object.
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<SourceTypeIdeal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<SourceTypeKlarna>,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<SourceTypeMultibanco>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///Information about the owner of the payment instrument that may be used or required by particular source types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<SourceTypeP24>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<SourceReceiverFlow>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<SourceRedirectFlow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<SourceTypeSepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<SourceTypeSofort>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_order: Option<SourceOrder>,
    ///Extra information about a source. This will appear on your customer's statement every time you charge the source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    ///The status of the source, one of `canceled`, `chargeable`, `consumed`, `failed`, or `pending`. Only `chargeable` sources can be used to create a charge.
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<SourceTypeThreeDSecure>,
    ///The `type` of the source. The `type` is a payment method, one of `ach_credit_transfer`, `ach_debit`, `alipay`, `bancontact`, `card`, `card_present`, `eps`, `giropay`, `ideal`, `multibanco`, `klarna`, `p24`, `sepa_debit`, `sofort`, `three_d_secure`, or `wechat`. An additional hash is included on the source with a name matching this value. It contains additional information specific to the [payment method](https://stripe.com/docs/sources) used.
    #[serde(rename = "type")]
    pub type_: String,
    ///Either `reusable` or `single_use`. Whether this source should be reusable or not. Some source types may or may not be reusable by construction, while others may leave the option at creation. If an incompatible value is passed, an error will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat: Option<SourceTypeWechat>,
}
impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}