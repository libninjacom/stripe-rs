use serde::{Serialize, Deserialize};
/**These bank accounts are payment methods on `Customer` objects.

On the other hand [External Accounts](https://stripe.com/docs/api#external_accounts) are transfer
destinations on `Account` objects for [Custom accounts](https://stripe.com/docs/connect/custom-accounts).
They can be bank accounts or debit cards as well, and are documented in the links above.

Related guide: [Bank debits and transfers](https://stripe.com/docs/payments/bank-debits-transfers)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankAccount {
    ///The ID of the account that the bank account is associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<serde_json::Value>,
    ///The name of the person or business that owns the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,
    ///The type of entity that holds the account. This can be either `individual` or `company`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<String>,
    ///The bank account type. This can only be `checking` or `savings` in most countries. In Japan, this can only be `futsu` or `toza`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    ///A set of available payout methods for this bank account. Only values from this set should be passed as the `method` when creating a payout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_payout_methods: Option<Vec<String>>,
    ///Name of the bank associated with the routing number (e.g., `WELLS FARGO`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    ///Two-letter ISO code representing the country the bank account is located in.
    pub country: String,
    ///Three-letter [ISO code for the currency](https://stripe.com/docs/payouts) paid out to the bank account.
    pub currency: String,
    ///The ID of the customer that the bank account is associated with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
    ///Whether this bank account is the default external account for its currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for_currency: Option<bool>,
    ///Uniquely identifies this particular bank account. You can use this attribute to check whether two bank accounts are the same.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    ///Information about the [upcoming new requirements for the bank account](https://stripe.com/docs/connect/custom-accounts/future-requirements), including what information needs to be collected, and by when.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<serde_json::Value>,
    ///Unique identifier for the object.
    pub id: String,
    ///The last four digits of the bank account number.
    pub last4: String,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///Information about the requirements for the bank account, including what information needs to be collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<serde_json::Value>,
    ///The routing transit number for the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
    /**For bank accounts, possible values are `new`, `validated`, `verified`, `verification_failed`, or `errored`. A bank account that hasn't had any activity or validation performed is `new`. If Stripe can determine that the bank account exists, its status will be `validated`. Note that there often isn’t enough information to know (e.g., for smaller credit unions), and the validation is not always run. If customer bank account verification has succeeded, the bank account status will be `verified`. If the verification failed for any reason, such as microdeposit failure, the status will be `verification_failed`. If a payout sent to this bank account fails, we'll set the status to `errored` and will not continue to send [scheduled payouts](https://stripe.com/docs/payouts#payout-schedule) until the bank details are updated.

For external accounts, possible values are `new`, `errored` and `verification_failed`. If a payouts fails, the status is set to `errored` and scheduled payouts are stopped until account details are updated. In India, if we can't [verify the owner of the bank account](https://support.stripe.com/questions/bank-account-ownership-verification), we'll set the status to `verification_failed`. Other validations aren't run against external accounts because they're only used for payouts. This means the other statuses don't apply.*/
    pub status: String,
}
impl std::fmt::Display for BankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}