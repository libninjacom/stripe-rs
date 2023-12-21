use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_prices_price`].

On request success, this will return a [`Price`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostPricesPriceRequest {
    pub price: String,
}
impl PostPricesPriceRequest {}
impl FluentRequest<'_, PostPricesPriceRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostPricesPriceRequest> {
    type Output = httpclient::InMemoryResult<Price>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/v1/prices/{price}", price = self.params.price);
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}