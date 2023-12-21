use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MandateBacsDebit {
    ///The status of the mandate on the Bacs network. Can be one of `pending`, `revoked`, `refused`, or `accepted`.
    pub network_status: String,
    ///The unique reference identifying the mandate on the Bacs network.
    pub reference: String,
    ///The URL that will contain the mandate that the customer has signed.
    pub url: String,
}
impl std::fmt::Display for MandateBacsDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}