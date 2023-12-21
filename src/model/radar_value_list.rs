use serde::{Serialize, Deserialize};
use super::RadarListListItemList;
/**Value lists allow you to group values together which can then be referenced in rules.

Related guide: [Default Stripe lists](https://stripe.com/docs/radar/lists#managing-list-items)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RadarValueList {
    ///The name of the value list for use in rules.
    pub alias: String,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///The name or email address of the user who created this value list.
    pub created_by: String,
    ///Unique identifier for the object.
    pub id: String,
    ///The type of items in the value list. One of `card_fingerprint`, `us_bank_account_fingerprint`, `sepa_debit_fingerprint`, `card_bin`, `email`, `ip_address`, `country`, `string`, `case_sensitive_string`, or `customer_id`.
    pub item_type: String,
    ///List of items contained within this value list.
    pub list_items: RadarListListItemList,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    ///The name of the value list.
    pub name: String,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for RadarValueList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}