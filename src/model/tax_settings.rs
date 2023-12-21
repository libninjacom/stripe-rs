use serde::{Serialize, Deserialize};
use super::{
    TaxProductResourceTaxSettingsDefaults, TaxProductResourceTaxSettingsStatusDetails,
};
/**You can use Tax `Settings` to manage configurations used by Stripe Tax calculations.

Related guide: [Using the Settings API](https://stripe.com/docs/tax/settings-api)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxSettings {
    ///
    pub defaults: TaxProductResourceTaxSettingsDefaults,
    ///The place where your business is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_office: Option<serde_json::Value>,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The `active` status indicates you have all required settings to calculate tax. A status can transition out of `active` when new required settings are introduced.
    pub status: String,
    ///
    pub status_details: TaxProductResourceTaxSettingsStatusDetails,
}
impl std::fmt::Display for TaxSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}