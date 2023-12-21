use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_products`].

On request success, this will return a [`Product`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostProductsRequest {}
impl PostProductsRequest {}
impl FluentRequest<'_, PostProductsRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostProductsRequest> {
    type Output = httpclient::InMemoryResult<Product>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/products";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}