use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryReceivedCreditsResourceLinkedFlows {
    ///The CreditReversal created as a result of this ReceivedCredit being reversed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_reversal: Option<String>,
    ///Set if the ReceivedCredit was created due to an [Issuing Authorization](https://stripe.com/docs/api#issuing_authorizations) object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_authorization: Option<String>,
    ///Set if the ReceivedCredit is also viewable as an [Issuing transaction](https://stripe.com/docs/api#issuing_transactions) object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_transaction: Option<String>,
    ///ID of the source flow. Set if `network` is `stripe` and the source flow is visible to the user. Examples of source flows include OutboundPayments, payouts, or CreditReversals.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_flow: Option<String>,
    ///The expandable object of the source flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_flow_details: Option<serde_json::Value>,
    ///The type of flow that originated the ReceivedCredit (for example, `outbound_payment`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_flow_type: Option<String>,
}
impl std::fmt::Display for TreasuryReceivedCreditsResourceLinkedFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}