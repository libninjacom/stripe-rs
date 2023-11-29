use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetCustomersCustomerSubscriptionsSubscriptionExposedIdRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub customer: String,
    pub expand: Option<Vec<String>>,
    pub subscription_exposed_id: String,
}
impl<'a> GetCustomersCustomerSubscriptionsSubscriptionExposedIdRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Subscription> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/v1/customers/{customer}/subscriptions/{subscription_exposed_id}",
                    customer = self.customer, subscription_exposed_id = self
                    .subscription_exposed_id
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.query("expand[]", &item.to_string());
            }
        }
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for GetCustomersCustomerSubscriptionsSubscriptionExposedIdRequest<'a> {
    type Output = httpclient::InMemoryResult<Subscription>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}