use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_shipping_rates`].

On request success, this will return a [`ShippingRate`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostShippingRatesRequest {}
impl PostShippingRatesRequest {}
impl FluentRequest<'_, PostShippingRatesRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostShippingRatesRequest> {
    type Output = httpclient::InMemoryResult<ShippingRate>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/shipping_rates";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}