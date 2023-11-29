use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DeleteCustomersCustomerSubscriptionsSubscriptionExposedIdRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub customer: String,
    pub subscription_exposed_id: String,
}
impl<'a> DeleteCustomersCustomerSubscriptionsSubscriptionExposedIdRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Subscription> {
        let mut r = self
            .http_client
            .client
            .delete(
                &format!(
                    "/v1/customers/{customer}/subscriptions/{subscription_exposed_id}",
                    customer = self.customer, subscription_exposed_id = self
                    .subscription_exposed_id
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture
for DeleteCustomersCustomerSubscriptionsSubscriptionExposedIdRequest<'a> {
    type Output = httpclient::InMemoryResult<Subscription>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}