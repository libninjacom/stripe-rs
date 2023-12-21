use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SourceOrderItem {
    ///The amount (price) for this order item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    ///This currency of this order item. Required when `amount` is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    ///Human-readable description for this order item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The ID of the associated object for this line item. Expandable if not null (e.g., expandable to a SKU).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    ///The quantity of this order item. When type is `sku`, this is the number of instances of the SKU to be ordered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    ///The type of this order item. Must be `sku`, `tax`, or `shipping`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl std::fmt::Display for SourceOrderItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}