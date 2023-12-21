use serde::{Serialize, Deserialize};
use super::{
    TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows,
    TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions,
};
///Use [InboundTransfers](https://stripe.com/docs/treasury/moving-money/financial-accounts/into/inbound-transfers) to add funds to your [FinancialAccount](https://stripe.com/docs/api#financial_accounts) via a PaymentMethod that is owned by you. The funds will be transferred via an ACH debit.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryInboundTransfer {
    ///Amount (in cents) transferred.
    pub amount: i64,
    ///Returns `true` if the InboundTransfer is able to be canceled.
    pub cancelable: bool,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Details about this InboundTransfer's failure. Only set when status is `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<serde_json::Value>,
    ///The FinancialAccount that received the funds.
    pub financial_account: String,
    ///A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_regulatory_receipt_url: Option<String>,
    ///Unique identifier for the object.
    pub id: String,
    ///
    pub linked_flows: TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The origin payment method to be debited for an InboundTransfer.
    pub origin_payment_method: String,
    ///Details about the PaymentMethod for an InboundTransfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_payment_method_details: Option<serde_json::Value>,
    ///Returns `true` if the funds for an InboundTransfer were returned after the InboundTransfer went to the `succeeded` state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned: Option<bool>,
    ///Statement descriptor shown when funds are debited from the source. Not all payment networks support `statement_descriptor`.
    pub statement_descriptor: String,
    ///Status of the InboundTransfer: `processing`, `succeeded`, `failed`, and `canceled`. An InboundTransfer is `processing` if it is created and pending. The status changes to `succeeded` once the funds have been "confirmed" and a `transaction` is created and posted. The status changes to `failed` if the transfer fails.
    pub status: String,
    ///
    pub status_transitions: TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions,
    ///The Transaction associated with this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<serde_json::Value>,
}
impl std::fmt::Display for TreasuryInboundTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}