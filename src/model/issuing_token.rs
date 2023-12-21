use serde::{Serialize, Deserialize};
use super::IssuingNetworkTokenNetworkData;
///An issuing token object is created when an issued card is added to a digital wallet. As a [card issuer](https://stripe.com/docs/issuing), you can [view and manage these tokens](https://stripe.com/docs/issuing/controls/token-management) through Stripe.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingToken {
    ///Card associated with this token.
    pub card: serde_json::Value,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///The hashed ID derived from the device ID from the card network associated with the token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_fingerprint: Option<String>,
    ///Unique identifier for the object.
    pub id: String,
    ///The last four digits of the token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///The token service provider / card network associated with the token.
    pub network: String,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_data: Option<IssuingNetworkTokenNetworkData>,
    ///Time at which the token was last updated by the card network. Measured in seconds since the Unix epoch.
    pub network_updated_at: i64,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The usage state of the token.
    pub status: String,
    ///The digital wallet for this token, if one was used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_provider: Option<String>,
}
impl std::fmt::Display for IssuingToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}