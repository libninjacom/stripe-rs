use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostPricesPriceRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub price: String,
}
impl<'a> PostPricesPriceRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Price> {
        let mut r = self
            .http_client
            .client
            .post(&format!("/v1/prices/{price}", price = self.price));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for PostPricesPriceRequest<'a> {
    type Output = httpclient::InMemoryResult<Price>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}