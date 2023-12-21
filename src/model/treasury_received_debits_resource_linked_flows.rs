use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryReceivedDebitsResourceLinkedFlows {
    ///The DebitReversal created as a result of this ReceivedDebit being reversed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_reversal: Option<String>,
    ///Set if the ReceivedDebit is associated with an InboundTransfer's return of funds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfer: Option<String>,
    ///Set if the ReceivedDebit was created due to an [Issuing Authorization](https://stripe.com/docs/api#issuing_authorizations) object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_authorization: Option<String>,
    ///Set if the ReceivedDebit is also viewable as an [Issuing Dispute](https://stripe.com/docs/api#issuing_disputes) object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_transaction: Option<String>,
}
impl std::fmt::Display for TreasuryReceivedDebitsResourceLinkedFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}