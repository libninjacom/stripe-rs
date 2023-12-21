use serde::{Serialize, Deserialize};
///A Financial Connections Account represents an account that exists outside of Stripe, to which you have been granted some degree of access.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FinancialConnectionsAccount {
    ///The account holder that this account belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder: Option<serde_json::Value>,
    ///The most recent information about the account's balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<serde_json::Value>,
    ///The state of the most recent attempt to refresh the account balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_refresh: Option<serde_json::Value>,
    ///The type of the account. Account category is further divided in `subcategory`.
    pub category: String,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///A human-readable name that has been assigned to this account, either by the account holder or by the institution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    ///Unique identifier for the object.
    pub id: String,
    ///The name of the institution that holds this account.
    pub institution_name: String,
    ///The last 4 digits of the account number. If present, this will be 4 numeric characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The most recent information about the account's owners.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership: Option<serde_json::Value>,
    ///The state of the most recent attempt to refresh the account owners.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_refresh: Option<serde_json::Value>,
    ///The list of permissions granted by this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    ///The status of the link to the account.
    pub status: String,
    /**If `category` is `cash`, one of:

 - `checking`
 - `savings`
 - `other`

If `category` is `credit`, one of:

 - `mortgage`
 - `line_of_credit`
 - `credit_card`
 - `other`

If `category` is `investment` or `other`, this will be `other`.*/
    pub subcategory: String,
    ///The list of data refresh subscriptions requested on this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<String>>,
    ///The [PaymentMethod type](https://stripe.com/docs/api/payment_methods/object#payment_method_object-type)(s) that can be created from this account.
    pub supported_payment_method_types: Vec<String>,
    ///The state of the most recent attempt to refresh the account transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_refresh: Option<serde_json::Value>,
}
impl std::fmt::Display for FinancialConnectionsAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}