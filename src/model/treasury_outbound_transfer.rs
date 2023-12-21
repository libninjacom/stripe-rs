use serde::{Serialize, Deserialize};
use super::{
    OutboundTransfersPaymentMethodDetails,
    TreasuryOutboundTransfersResourceStatusTransitions,
};
/**Use OutboundTransfers to transfer funds from a [FinancialAccount](https://stripe.com/docs/api#financial_accounts) to a PaymentMethod belonging to the same entity. To send funds to a different party, use [OutboundPayments](https://stripe.com/docs/api#outbound_payments) instead. You can send funds over ACH rails or through a domestic wire transfer to a user's own external bank account.

Simulate OutboundTransfer state changes with the `/v1/test_helpers/treasury/outbound_transfers` endpoints. These methods can only be called on test mode objects.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryOutboundTransfer {
    ///Amount (in cents) transferred.
    pub amount: i64,
    ///Returns `true` if the object can be canceled, and `false` otherwise.
    pub cancelable: bool,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The PaymentMethod used as the payment instrument for an OutboundTransfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment_method: Option<String>,
    ///
    pub destination_payment_method_details: OutboundTransfersPaymentMethodDetails,
    ///The date when funds are expected to arrive in the destination account.
    pub expected_arrival_date: i64,
    ///The FinancialAccount that funds were pulled from.
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
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///Details about a returned OutboundTransfer. Only set when the status is `returned`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_details: Option<serde_json::Value>,
    ///Information about the OutboundTransfer to be sent to the recipient account.
    pub statement_descriptor: String,
    ///Current status of the OutboundTransfer: `processing`, `failed`, `canceled`, `posted`, `returned`. An OutboundTransfer is `processing` if it has been created and is pending. The status changes to `posted` once the OutboundTransfer has been "confirmed" and funds have left the account, or to `failed` or `canceled`. If an OutboundTransfer fails to arrive at its destination, its status will change to `returned`.
    pub status: String,
    ///
    pub status_transitions: TreasuryOutboundTransfersResourceStatusTransitions,
    ///The Transaction associated with this object.
    pub transaction: serde_json::Value,
}
impl std::fmt::Display for TreasuryOutboundTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}