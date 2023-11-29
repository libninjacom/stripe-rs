
use serde::{Serialize, Deserialize};
use super::FeeRefundList;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationFee {
    pub account: serde_json::Value,
    pub amount: i64,
    pub amount_refunded: i64,
    pub application: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<serde_json::Value>,
    pub charge: serde_json::Value,
    pub created: i64,
    pub currency: String,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originating_transaction: Option<serde_json::Value>,
    pub refunded: bool,
    pub refunds: FeeRefundList,
}
impl std::fmt::Display for ApplicationFee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}