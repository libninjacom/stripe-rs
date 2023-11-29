use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostCustomersCustomerSubscriptionsSubscriptionExposedIdRequest {
    pub customer: String,
    pub subscription_exposed_id: String,
}
impl PostCustomersCustomerSubscriptionsSubscriptionExposedIdRequest {}
impl FluentRequest<'_, PostCustomersCustomerSubscriptionsSubscriptionExposedIdRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostCustomersCustomerSubscriptionsSubscriptionExposedIdRequest> {
    type Output = httpclient::InMemoryResult<Subscription>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = &format!(
                "/v1/customers/{customer}/subscriptions/{subscription_exposed_id}",
                customer = self.params.customer, subscription_exposed_id = self.params
                .subscription_exposed_id
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}