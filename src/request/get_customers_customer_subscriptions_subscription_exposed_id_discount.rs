use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_customers_customer_subscriptions_subscription_exposed_id_discount`].

On request success, this will return a [`Discount`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCustomersCustomerSubscriptionsSubscriptionExposedIdDiscountRequest {
    pub customer: String,
    pub expand: Option<Vec<String>>,
    pub subscription_exposed_id: String,
}
impl GetCustomersCustomerSubscriptionsSubscriptionExposedIdDiscountRequest {}
impl FluentRequest<
    '_,
    GetCustomersCustomerSubscriptionsSubscriptionExposedIdDiscountRequest,
> {
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<
    'a,
    GetCustomersCustomerSubscriptionsSubscriptionExposedIdDiscountRequest,
> {
    type Output = httpclient::InMemoryResult<Discount>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/customers/{customer}/subscriptions/{subscription_exposed_id}/discount",
                customer = self.params.customer, subscription_exposed_id = self.params
                .subscription_exposed_id
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}