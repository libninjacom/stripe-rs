use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteCustomersCustomerSubscriptionsSubscriptionExposedIdDiscountRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub customer: String,
    pub subscription_exposed_id: String,
}
impl<'a> DeleteCustomersCustomerSubscriptionsSubscriptionExposedIdDiscountRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedDiscount> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v1/customers/{customer}/subscriptions/{subscription_exposed_id}/discount",
                    customer = self.customer, subscription_exposed_id = self
                    .subscription_exposed_id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
