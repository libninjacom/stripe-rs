use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetShippingRatesShippingRateTokenRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub shipping_rate_token: String,
}
impl<'a> GetShippingRatesShippingRateTokenRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ShippingRate> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/v1/shipping_rates/{shipping_rate_token}", shipping_rate_token =
                    self.shipping_rate_token
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
impl<'a> ::std::future::IntoFuture for GetShippingRatesShippingRateTokenRequest<'a> {
    type Output = httpclient::InMemoryResult<ShippingRate>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}