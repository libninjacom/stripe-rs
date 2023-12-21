use serde::{Serialize, Deserialize};
use super::{BalanceTransaction, IssuingDisputeEvidence};
/**As a [card issuer](https://stripe.com/docs/issuing), you can dispute transactions that the cardholder does not recognize, suspects to be fraudulent, or has other issues with.

Related guide: [Issuing disputes](https://stripe.com/docs/issuing/purchases/disputes)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingDispute {
    ///Disputed amount in the card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal). Usually the amount of the `transaction`, but can differ (usually because of currency fluctuation).
    pub amount: i64,
    ///List of balance transactions associated with the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transactions: Option<Vec<BalanceTransaction>>,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///The currency the `transaction` was made in.
    pub currency: String,
    ///
    pub evidence: IssuingDisputeEvidence,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///Current status of the dispute.
    pub status: String,
    ///The transaction being disputed.
    pub transaction: serde_json::Value,
    ///[Treasury](https://stripe.com/docs/api/treasury) details related to this dispute if it was created on a [FinancialAccount](/docs/api/treasury/financial_accounts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<serde_json::Value>,
}
impl std::fmt::Display for IssuingDispute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}