
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountDeclineChargeOn {
    pub avs_failure: bool,
    pub cvc_failure: bool,
}
impl std::fmt::Display for AccountDeclineChargeOn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}