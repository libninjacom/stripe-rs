use serde::{Serialize, Deserialize};
/**Value list items allow you to add specific values to a given Radar value list, which can then be used in rules.

Related guide: [Managing list items](https://stripe.com/docs/radar/lists#managing-list-items)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RadarValueListItem {
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///The name or email address of the user who added this item to the value list.
    pub created_by: String,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The value of the item.
    pub value: String,
    ///The identifier of the value list this item belongs to.
    pub value_list: String,
}
impl std::fmt::Display for RadarValueListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}