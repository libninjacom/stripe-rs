
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuingSettlement {
    pub bin: String,
    pub clearing_date: i64,
    pub created: i64,
    pub currency: String,
    pub id: String,
    pub interchange_fees: i64,
    pub livemode: bool,
    pub metadata: serde_json::Value,
    pub net_total: i64,
    pub network: String,
    pub network_fees: i64,
    pub network_settlement_identifier: String,
    pub object: String,
    pub settlement_service: String,
    pub transaction_count: i64,
    pub transaction_volume: i64,
}
impl std::fmt::Display for IssuingSettlement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}