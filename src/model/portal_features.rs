use serde::{Serialize, Deserialize};
use super::{
    PortalCustomerUpdate, PortalInvoiceList, PortalPaymentMethodUpdate,
    PortalSubscriptionCancel, PortalSubscriptionPause, PortalSubscriptionUpdate,
};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortalFeatures {
    ///
    pub customer_update: PortalCustomerUpdate,
    ///
    pub invoice_history: PortalInvoiceList,
    ///
    pub payment_method_update: PortalPaymentMethodUpdate,
    ///
    pub subscription_cancel: PortalSubscriptionCancel,
    ///
    pub subscription_pause: PortalSubscriptionPause,
    ///
    pub subscription_update: PortalSubscriptionUpdate,
}
impl std::fmt::Display for PortalFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}