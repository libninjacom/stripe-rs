use serde::{Serialize, Deserialize};
/**A Connection Token is used by the Stripe Terminal SDK to connect to a reader.

Related guide: [Fleet management](https://stripe.com/docs/terminal/fleet/locations)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminalConnectionToken {
    ///The id of the location that this connection token is scoped to. Note that location scoping only applies to internet-connected readers. For more details, see [the docs on scoping connection tokens](https://stripe.com/docs/terminal/fleet/locations#connection-tokens).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///Your application should pass this token to the Stripe Terminal SDK.
    pub secret: String,
}
impl std::fmt::Display for TerminalConnectionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}