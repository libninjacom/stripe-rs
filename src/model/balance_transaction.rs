use serde::{Serialize, Deserialize};
use super::Fee;
/**Balance transactions represent funds moving through your Stripe account.
Stripe creates them for every type of transaction that enters or leaves your Stripe account balance.

Related guide: [Balance transaction types](https://stripe.com/docs/reports/balance-transaction-types)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BalanceTransaction {
    ///Gross amount of this transaction (in cents (or local equivalent)). A positive value represents funds charged to another party, and a negative value represents funds sent to another party.
    pub amount: i64,
    ///The date that the transaction's net funds become available in the Stripe balance.
    pub available_on: i64,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///If applicable, this transaction uses an exchange rate. If money converts from currency A to currency B, then the `amount` in currency A, multipled by the `exchange_rate`, equals the `amount` in currency B. For example, if you charge a customer 10.00 EUR, the PaymentIntent's `amount` is `1000` and `currency` is `eur`. If this converts to 12.34 USD in your Stripe account, the BalanceTransaction's `amount` is `1234`, its `currency` is `usd`, and the `exchange_rate` is `1.234`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_rate: Option<f64>,
    ///Fees (in cents (or local equivalent)) paid for this transaction. Represented as a positive integer when assessed.
    pub fee: i64,
    ///Detailed breakdown of fees (in cents (or local equivalent)) paid for this transaction.
    pub fee_details: Vec<Fee>,
    ///Unique identifier for the object.
    pub id: String,
    ///Net impact to a Stripe balance (in cents (or local equivalent)). A positive value represents incrementing a Stripe balance, and a negative value decrementing a Stripe balance. You can calculate the net impact of a transaction on a balance by `amount` - `fee`
    pub net: i64,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///Learn more about how [reporting categories](https://stripe.com/docs/reports/reporting-categories) can help you understand balance transactions from an accounting perspective.
    pub reporting_category: String,
    ///This transaction relates to the Stripe object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<serde_json::Value>,
    ///The transaction's net funds status in the Stripe balance, which are either `available` or `pending`.
    pub status: String,
    ///Transaction type: `adjustment`, `advance`, `advance_funding`, `anticipation_repayment`, `application_fee`, `application_fee_refund`, `charge`, `climate_order_purchase`, `climate_order_refund`, `connect_collection_transfer`, `contribution`, `issuing_authorization_hold`, `issuing_authorization_release`, `issuing_dispute`, `issuing_transaction`, `obligation_inbound`, `obligation_outbound`, `obligation_reversal_inbound`, `obligation_reversal_outbound`, `obligation_payout`, `obligation_payout_failure`, `payment`, `payment_failure_refund`, `payment_network_reserve_hold`, `payment_network_reserve_release`, `payment_refund`, `payment_reversal`, `payment_unreconciled`, `payout`, `payout_cancel`, `payout_failure`, `refund`, `refund_failure`, `reserve_transaction`, `reserved_funds`, `stripe_fee`, `stripe_fx_fee`, `tax_fee`, `topup`, `topup_reversal`, `transfer`, `transfer_cancel`, `transfer_failure`, or `transfer_refund`. Learn more about [balance transaction types and what they represent](https://stripe.com/docs/reports/balance-transaction-types). To classify transactions for accounting purposes, consider `reporting_category` instead.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for BalanceTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}