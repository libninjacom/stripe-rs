use serde::{Serialize, Deserialize};
use super::TaxProductRegistrationsResourceCountryOptions;
/**A Tax `Registration` lets us know that your business is registered to collect tax on payments within a region, enabling you to [automatically collect tax](https://stripe.com/docs/tax).

Stripe doesn't register on your behalf with the relevant authorities when you create a Tax `Registration` object. For more information on how to register to collect tax, see [our guide](https://stripe.com/docs/tax/registering).

Related guide: [Using the Registrations API](https://stripe.com/docs/tax/registrations-api)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxRegistration {
    ///Time at which the registration becomes active. Measured in seconds since the Unix epoch.
    pub active_from: i64,
    ///Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    ///
    pub country_options: TaxProductRegistrationsResourceCountryOptions,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///If set, the registration stops being active at this time. If not set, the registration will be active indefinitely. Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The status of the registration. This field is present for convenience and can be deduced from `active_from` and `expires_at`.
    pub status: String,
}
impl std::fmt::Display for TaxRegistration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}