use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_shipping_rates_shipping_rate_token`].

On request success, this will return a [`ShippingRate`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetShippingRatesShippingRateTokenRequest {
    pub expand: Option<Vec<String>>,
    pub shipping_rate_token: String,
}
impl GetShippingRatesShippingRateTokenRequest {}
impl FluentRequest<'_, GetShippingRatesShippingRateTokenRequest> {
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetShippingRatesShippingRateTokenRequest> {
    type Output = httpclient::InMemoryResult<ShippingRate>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/shipping_rates/{shipping_rate_token}", shipping_rate_token = self
                .params.shipping_rate_token
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}