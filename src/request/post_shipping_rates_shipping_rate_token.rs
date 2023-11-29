use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostShippingRatesShippingRateTokenRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub shipping_rate_token: String,
}
impl<'a> PostShippingRatesShippingRateTokenRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ShippingRate> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v1/shipping_rates/{shipping_rate_token}", shipping_rate_token =
                    self.shipping_rate_token
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for PostShippingRatesShippingRateTokenRequest<'a> {
    type Output = httpclient::InMemoryResult<ShippingRate>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}