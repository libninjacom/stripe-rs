use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodConfigResourceDisplayPreference {
    ///For child configs, whether or not the account's preference will be observed. If `false`, the parent configuration's default is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overridable: Option<bool>,
    ///The account's display preference.
    pub preference: String,
    ///The effective display preference value.
    pub value: String,
}
impl std::fmt::Display for PaymentMethodConfigResourceDisplayPreference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}