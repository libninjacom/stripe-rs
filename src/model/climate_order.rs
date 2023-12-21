use serde::{Serialize, Deserialize};
use super::{ClimateRemovalsBeneficiary, ClimateRemovalsOrderDeliveries};
/**Orders represent your intent to purchase a particular Climate product. When you create an order, the
payment is deducted from your merchant balance.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClimateOrder {
    ///Total amount of [Frontier](https://frontierclimate.com/)'s service fees in the currency's smallest unit.
    pub amount_fees: i64,
    ///Total amount of the carbon removal in the currency's smallest unit.
    pub amount_subtotal: i64,
    ///Total amount of the order including fees in the currency's smallest unit.
    pub amount_total: i64,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary: Option<ClimateRemovalsBeneficiary>,
    ///Time at which the order was canceled. Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<i64>,
    ///Reason for the cancellation of this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    ///For delivered orders, a URL to a delivery certificate for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    ///Time at which the order was confirmed. Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmed_at: Option<i64>,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase, representing the currency for this order.
    pub currency: String,
    ///Time at which the order's expected_delivery_year was delayed. Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delayed_at: Option<i64>,
    ///Time at which the order was delivered. Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered_at: Option<i64>,
    ///Details about the delivery of carbon removal for this order.
    pub delivery_details: Vec<ClimateRemovalsOrderDeliveries>,
    ///The year this order is expected to be delivered.
    pub expected_delivery_year: i64,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    ///Quantity of carbon removal that is included in this order.
    #[serde(with = "rust_decimal::serde::str")]
    pub metric_tons: rust_decimal::Decimal,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///Unique ID for the Climate `Product` this order is purchasing.
    pub product: serde_json::Value,
    ///Time at which the order's product was substituted for a different product. Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_substituted_at: Option<i64>,
    ///The current status of this order.
    pub status: String,
}
impl std::fmt::Display for ClimateOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}