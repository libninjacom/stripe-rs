use serde::{Serialize, Deserialize};
use super::ClimateSupplier;
/**A Climate product represents a type of carbon removal unit available for reservation.
You can retrieve it to see the current price and availability.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClimateProduct {
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Current prices for a metric ton of carbon removal in a currency's smallest unit.
    pub current_prices_per_metric_ton: serde_json::Value,
    ///The year in which the carbon removal is expected to be delivered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_year: Option<i64>,
    /**Unique identifier for the object. For convenience, Climate product IDs are human-readable strings
that start with `climsku_`. See [carbon removal inventory](https://stripe.com/docs/climate/orders/carbon-removal-inventory)
for a list of available carbon removal products.*/
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///The quantity of metric tons available for reservation.
    #[serde(with = "rust_decimal::serde::str")]
    pub metric_tons_available: rust_decimal::Decimal,
    ///The Climate product's name.
    pub name: String,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The carbon removal suppliers that fulfill orders for this Climate product.
    pub suppliers: Vec<ClimateSupplier>,
}
impl std::fmt::Display for ClimateProduct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}