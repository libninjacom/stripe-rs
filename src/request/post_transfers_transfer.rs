use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_transfers_transfer`].

On request success, this will return a [`Transfer`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTransfersTransferRequest {
    pub transfer: String,
}
impl PostTransfersTransferRequest {}
impl FluentRequest<'_, PostTransfersTransferRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostTransfersTransferRequest> {
    type Output = httpclient::InMemoryResult<Transfer>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/transfers/{transfer}", transfer = self.params.transfer
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}