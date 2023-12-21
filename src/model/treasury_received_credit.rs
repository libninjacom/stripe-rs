use serde::{Serialize, Deserialize};
use super::{
    TreasuryReceivedCreditsResourceLinkedFlows,
    TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails,
};
///ReceivedCredits represent funds sent to a [FinancialAccount](https://stripe.com/docs/api#financial_accounts) (for example, via ACH or wire). These money movements are not initiated from the FinancialAccount.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryReceivedCredit {
    ///Amount (in cents) transferred.
    pub amount: i64,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: String,
    ///Reason for the failure. A ReceivedCredit might fail because the receiving FinancialAccount is closed or frozen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    ///The FinancialAccount that received the funds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<String>,
    ///A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_regulatory_receipt_url: Option<String>,
    ///Unique identifier for the object.
    pub id: String,
    ///
    pub initiating_payment_method_details: TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails,
    ///
    pub linked_flows: TreasuryReceivedCreditsResourceLinkedFlows,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///The rails used to send the funds.
    pub network: String,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///Details describing when a ReceivedCredit may be reversed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversal_details: Option<serde_json::Value>,
    ///Status of the ReceivedCredit. ReceivedCredits are created either `succeeded` (approved) or `failed` (declined). If a ReceivedCredit is declined, the failure reason can be found in the `failure_code` field.
    pub status: String,
    ///The Transaction associated with this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<serde_json::Value>,
}
impl std::fmt::Display for TreasuryReceivedCredit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}