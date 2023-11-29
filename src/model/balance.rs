
use serde::{Serialize, Deserialize};
use super::{BalanceAmount, BalanceAmountNet, BalanceDetail};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Balance {
    pub available: Vec<BalanceAmount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_reserved: Option<Vec<BalanceAmount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant_available: Option<Vec<BalanceAmountNet>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing: Option<BalanceDetail>,
    pub livemode: bool,
    pub object: String,
    pub pending: Vec<BalanceAmount>,
}
impl std::fmt::Display for Balance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}