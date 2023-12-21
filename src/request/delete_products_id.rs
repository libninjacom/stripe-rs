use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::delete_products_id`].

On request success, this will return a [`DeletedProduct`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteProductsIdRequest {
    pub id: String,
}
impl DeleteProductsIdRequest {}
impl FluentRequest<'_, DeleteProductsIdRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, DeleteProductsIdRequest> {
    type Output = httpclient::InMemoryResult<DeletedProduct>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/v1/products/{id}", id = self.params.id);
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}