use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SourceReceiverFlow {
    ///The address of the receiver source. This is the value that should be communicated to the customer to send their funds to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    ///The total amount that was moved to your balance. This is almost always equal to the amount charged. In rare cases when customers deposit excess funds and we are unable to refund those, those funds get moved to your balance and show up in amount_charged as well. The amount charged is expressed in the source's currency.
    pub amount_charged: i64,
    ///The total amount received by the receiver source. `amount_received = amount_returned + amount_charged` should be true for consumed sources unless customers deposit excess funds. The amount received is expressed in the source's currency.
    pub amount_received: i64,
    ///The total amount that was returned to the customer. The amount returned is expressed in the source's currency.
    pub amount_returned: i64,
    ///Type of refund attribute method, one of `email`, `manual`, or `none`.
    pub refund_attributes_method: String,
    ///Type of refund attribute status, one of `missing`, `requested`, or `available`.
    pub refund_attributes_status: String,
}
impl std::fmt::Display for SourceReceiverFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}