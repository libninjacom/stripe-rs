
use serde::{Serialize, Deserialize};
use super::{
    CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft,
    CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction,
    CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction,
    CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction,
    CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalance,
    CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransaction,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerCashBalanceTransaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjusted_for_overdraft: Option<
        CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_to_payment: Option<
        CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction,
    >,
    pub created: i64,
    pub currency: String,
    pub customer: serde_json::Value,
    pub ending_balance: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funded: Option<
        CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction,
    >,
    pub id: String,
    pub livemode: bool,
    pub net_amount: i64,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_from_payment: Option<
        CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transferred_to_balance: Option<
        CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalance,
    >,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unapplied_from_payment: Option<
        CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransaction,
    >,
}
impl std::fmt::Display for CustomerCashBalanceTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}