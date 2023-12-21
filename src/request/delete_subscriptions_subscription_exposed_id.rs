use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::delete_subscriptions_subscription_exposed_id`].

On request success, this will return a [`Subscription`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSubscriptionsSubscriptionExposedIdRequest {
    pub subscription_exposed_id: String,
}
impl DeleteSubscriptionsSubscriptionExposedIdRequest {}
impl FluentRequest<'_, DeleteSubscriptionsSubscriptionExposedIdRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, DeleteSubscriptionsSubscriptionExposedIdRequest> {
    type Output = httpclient::InMemoryResult<Subscription>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/subscriptions/{subscription_exposed_id}", subscription_exposed_id =
                self.params.subscription_exposed_id
            );
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}