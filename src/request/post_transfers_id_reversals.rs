use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_transfers_id_reversals`].

On request success, this will return a [`TransferReversal`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTransfersIdReversalsRequest {
    pub id: String,
}
impl PostTransfersIdReversalsRequest {}
impl FluentRequest<'_, PostTransfersIdReversalsRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostTransfersIdReversalsRequest> {
    type Output = httpclient::InMemoryResult<TransferReversal>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/v1/transfers/{id}/reversals", id = self.params.id);
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}