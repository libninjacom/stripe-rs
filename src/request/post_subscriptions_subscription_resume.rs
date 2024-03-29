use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_subscriptions_subscription_resume`].

On request success, this will return a [`Subscription`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostSubscriptionsSubscriptionResumeRequest {
    pub subscription: String,
}
impl PostSubscriptionsSubscriptionResumeRequest {}
impl FluentRequest<'_, PostSubscriptionsSubscriptionResumeRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostSubscriptionsSubscriptionResumeRequest> {
    type Output = httpclient::InMemoryResult<Subscription>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/subscriptions/{subscription}/resume", subscription = self.params
                .subscription
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}