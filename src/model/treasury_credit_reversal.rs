use serde::{Serialize, Deserialize};
use super::TreasuryReceivedCreditsResourceStatusTransitions;
///You can reverse some [ReceivedCredits](https://stripe.com/docs/api#received_credits) depending on their network and source flow. Reversing a ReceivedCredit leads to the creation of a new object known as a CreditReversal.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryCreditReversal {
    ///Amount (in cents) transferred.
    pub amount: i64,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///The FinancialAccount to reverse funds from.
    pub financial_account: String,
    ///A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_regulatory_receipt_url: Option<String>,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    ///The rails used to reverse the funds.
    pub network: String,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The ReceivedCredit being reversed.
    pub received_credit: String,
    ///Status of the CreditReversal
    pub status: String,
    ///
    pub status_transitions: TreasuryReceivedCreditsResourceStatusTransitions,
    ///The Transaction associated with this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<serde_json::Value>,
}
impl std::fmt::Display for TreasuryCreditReversal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}