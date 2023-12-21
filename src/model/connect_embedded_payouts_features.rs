use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectEmbeddedPayoutsFeatures {
    ///Whether to allow payout schedule to be changed. Default `true` when Stripe owns Loss Liability, default `false` otherwise.
    pub edit_payout_schedule: bool,
    ///Whether to allow creation of instant payouts. Default `true` when Stripe owns Loss Liability, default `false` otherwise.
    pub instant_payouts: bool,
    ///Whether to allow creation of standard payouts. Default `true` when Stripe owns Loss Liability, default `false` otherwise.
    pub standard_payouts: bool,
}
impl std::fmt::Display for ConnectEmbeddedPayoutsFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}