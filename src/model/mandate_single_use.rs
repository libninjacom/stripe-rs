use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MandateSingleUse {
    ///The amount of the payment on a single use mandate.
    pub amount: i64,
    ///The currency of the payment on a single use mandate.
    pub currency: String,
}
impl std::fmt::Display for MandateSingleUse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}