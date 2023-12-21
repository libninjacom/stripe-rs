use serde::{Serialize, Deserialize};
use super::{IssuingCardAuthorizationControls, IssuingCardholder};
///You can [create physical or virtual cards](https://stripe.com/docs/issuing/cards) that are issued to cardholders.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingCard {
    ///The brand of the card.
    pub brand: String,
    ///The reason why the card was canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    /**An Issuing `Cardholder` object represents an individual or business entity who is [issued](https://stripe.com/docs/issuing) cards.

Related guide: [How to create a cardholder](https://stripe.com/docs/issuing/cards#create-cardholder)*/
    pub cardholder: IssuingCardholder,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Supported currencies are `usd` in the US, `eur` in the EU, and `gbp` in the UK.
    pub currency: String,
    ///The card's CVC. For security reasons, this is only available for virtual cards, and will be omitted unless you explicitly request it with [the `expand` parameter](https://stripe.com/docs/api/expanding_objects). Additionally, it's only available via the ["Retrieve a card" endpoint](https://stripe.com/docs/api/issuing/cards/retrieve), not via "List all cards" or any other endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc: Option<String>,
    ///The expiration month of the card.
    pub exp_month: i64,
    ///The expiration year of the card.
    pub exp_year: i64,
    ///The financial account this card is attached to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<String>,
    ///Unique identifier for the object.
    pub id: String,
    ///The last 4 digits of the card number.
    pub last4: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    ///The full unredacted card number. For security reasons, this is only available for virtual cards, and will be omitted unless you explicitly request it with [the `expand` parameter](https://stripe.com/docs/api/expanding_objects). Additionally, it's only available via the ["Retrieve a card" endpoint](https://stripe.com/docs/api/issuing/cards/retrieve), not via "List all cards" or any other endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The latest card that replaces this card, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced_by: Option<serde_json::Value>,
    ///The card this card replaces, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_for: Option<serde_json::Value>,
    ///The reason why the previous card needed to be replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_reason: Option<String>,
    ///Where and how the card will be shipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<serde_json::Value>,
    ///
    pub spending_controls: IssuingCardAuthorizationControls,
    ///Whether authorizations can be approved on this card. May be blocked from activating cards depending on past-due Cardholder requirements. Defaults to `inactive`.
    pub status: String,
    ///The type of the card.
    #[serde(rename = "type")]
    pub type_: String,
    ///Information relating to digital wallets (like Apple Pay and Google Pay).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallets: Option<serde_json::Value>,
}
impl std::fmt::Display for IssuingCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}