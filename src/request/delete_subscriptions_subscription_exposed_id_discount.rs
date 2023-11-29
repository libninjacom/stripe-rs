use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DeleteSubscriptionsSubscriptionExposedIdDiscountRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub subscription_exposed_id: String,
}
impl<'a> DeleteSubscriptionsSubscriptionExposedIdDiscountRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<DeletedDiscount> {
        let mut r = self
            .http_client
            .client
            .delete(
                &format!(
                    "/v1/subscriptions/{subscription_exposed_id}/discount",
                    subscription_exposed_id = self.subscription_exposed_id
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture
for DeleteSubscriptionsSubscriptionExposedIdDiscountRequest<'a> {
    type Output = httpclient::InMemoryResult<DeletedDiscount>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}