use serde::{Serialize, Deserialize};
use super::{IssuingCardholderAddress, IssuingCardholderRequirements};
/**An Issuing `Cardholder` object represents an individual or business entity who is [issued](https://stripe.com/docs/issuing) cards.

Related guide: [How to create a cardholder](https://stripe.com/docs/issuing/cards#create-cardholder)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingCardholder {
    ///
    pub billing: IssuingCardholderAddress,
    ///Additional information about a `company` cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<serde_json::Value>,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///The cardholder's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    ///Unique identifier for the object.
    pub id: String,
    ///Additional information about an `individual` cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<serde_json::Value>,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    ///The cardholder's name. This will be printed on cards issued to them.
    pub name: String,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The cardholder's phone number. This is required for all cardholders who will be creating EU cards. See the [3D Secure documentation](https://stripe.com/docs/issuing/3d-secure#when-is-3d-secure-applied) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /**The cardholder’s preferred locales (languages), ordered by preference. Locales can be `de`, `en`, `es`, `fr`, or `it`.
 This changes the language of the [3D Secure flow](https://stripe.com/docs/issuing/3d-secure) and one-time password messages sent to the cardholder.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Vec<String>>,
    ///
    pub requirements: IssuingCardholderRequirements,
    ///Rules that control spending across this cardholder's cards. Refer to our [documentation](https://stripe.com/docs/issuing/controls/spending-controls) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_controls: Option<serde_json::Value>,
    ///Specifies whether to permit authorizations on this cardholder's cards.
    pub status: String,
    ///One of `individual` or `company`. See [Choose a cardholder type](https://stripe.com/docs/issuing/other/choose-cardholder) for more details.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for IssuingCardholder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}