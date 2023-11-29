
use serde::{Serialize, Deserialize};
use super::TreasuryOutboundPayment;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentList {
    pub data: Vec<TreasuryOutboundPayment>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for TreasuryOutboundPaymentsResourceOutboundPaymentList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}