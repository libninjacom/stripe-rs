use serde::{Serialize, Deserialize};
use super::CountrySpecVerificationFields;
/**Stripe needs to collect certain pieces of information about each account
created. These requirements can differ depending on the account's country. The
Country Specs API makes these rules available to your integration.

You can also view the information from this API call as [an online
guide](/docs/connect/required-verification-information).*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CountrySpec {
    ///The default currency for this country. This applies to both payment methods and bank accounts.
    pub default_currency: String,
    ///Unique identifier for the object. Represented as the ISO country code for this country.
    pub id: String,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///Currencies that can be accepted in the specific country (for transfers).
    pub supported_bank_account_currencies: serde_json::Value,
    ///Currencies that can be accepted in the specified country (for payments).
    pub supported_payment_currencies: Vec<String>,
    ///Payment methods available in the specified country. You may need to enable some payment methods (e.g., [ACH](https://stripe.com/docs/ach)) on your account before they appear in this list. The `stripe` payment method refers to [charging through your platform](https://stripe.com/docs/connect/destination-charges).
    pub supported_payment_methods: Vec<String>,
    ///Countries that can accept transfers from the specified country.
    pub supported_transfer_countries: Vec<String>,
    ///
    pub verification_fields: CountrySpecVerificationFields,
}
impl std::fmt::Display for CountrySpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}