use serde::{Serialize, Deserialize};
use super::{
    Address, LegalEntityDob, LegalEntityPersonVerification,
    PersonAdditionalTosAcceptances, PersonRelationship,
};
/**This is an object representing a person associated with a Stripe account.

A platform cannot access a Standard or Express account's persons after the account starts onboarding, such as after generating an account link for the account.
See the [Standard onboarding](https://stripe.com/docs/connect/standard-accounts) or [Express onboarding documentation](https://stripe.com/docs/connect/express-accounts) for information about platform prefilling and account onboarding steps.

Related guide: [Handling identity verification with the API](https://stripe.com/docs/connect/handling-api-verification#person-information)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Person {
    ///The account the person is associated with.
    pub account: String,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_tos_acceptances: Option<PersonAdditionalTosAcceptances>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<serde_json::Value>,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<LegalEntityDob>,
    ///The person's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    ///The person's first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    ///The Kana variation of the person's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kana: Option<String>,
    ///The Kanji variation of the person's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kanji: Option<String>,
    ///A list of alternate names or aliases that the person is known by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name_aliases: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<serde_json::Value>,
    ///The person's gender (International regulations require either "male" or "female").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    ///Unique identifier for the object.
    pub id: String,
    ///Whether the person's `id_number` was provided. True if either the full ID number was provided or if only the required part of the ID number was provided (ex. last four of an individual's SSN for the US indicated by `ssn_last_4_provided`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number_provided: Option<bool>,
    ///Whether the person's `id_number_secondary` was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number_secondary_provided: Option<bool>,
    ///The person's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    ///The Kana variation of the person's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kana: Option<String>,
    ///The Kanji variation of the person's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kanji: Option<String>,
    ///The person's maiden name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maiden_name: Option<String>,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    ///The country where the person is a national.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The person's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    ///Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<Address>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<PersonRelationship>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<serde_json::Value>,
    ///Whether the last four digits of the person's Social Security number have been provided (U.S. only).
    #[serde(rename = "ssn_last_4_provided")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last4_provided: Option<bool>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<LegalEntityPersonVerification>,
}
impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}