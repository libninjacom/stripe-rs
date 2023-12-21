use serde::{Serialize, Deserialize};
///When a non-stripe BIN is used, any use of an [issued card](https://stripe.com/docs/issuing) must be settled directly with the card network. The net amount owed is represented by an Issuing `Settlement` object.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingSettlement {
    ///The Bank Identification Number reflecting this settlement record.
    pub bin: String,
    ///The date that the transactions are cleared and posted to user's accounts.
    pub clearing_date: i64,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///Unique identifier for the object.
    pub id: String,
    ///The total interchange received as reimbursement for the transactions.
    pub interchange_fees: i64,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    ///The total net amount required to settle with the network.
    pub net_total: i64,
    ///The card network for this settlement report. One of ["visa"]
    pub network: String,
    ///The total amount of fees owed to the network.
    pub network_fees: i64,
    ///The Settlement Identification Number assigned by the network.
    pub network_settlement_identifier: String,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///One of `international` or `uk_national_net`.
    pub settlement_service: String,
    ///The total number of transactions reflected in this settlement.
    pub transaction_count: i64,
    ///The total transaction amount reflected in this settlement.
    pub transaction_volume: i64,
}
impl std::fmt::Display for IssuingSettlement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}