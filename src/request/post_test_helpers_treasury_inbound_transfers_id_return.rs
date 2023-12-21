use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_test_helpers_treasury_inbound_transfers_id_return`].

On request success, this will return a [`TreasuryInboundTransfer`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTestHelpersTreasuryInboundTransfersIdReturnRequest {
    pub id: String,
}
impl PostTestHelpersTreasuryInboundTransfersIdReturnRequest {}
impl FluentRequest<'_, PostTestHelpersTreasuryInboundTransfersIdReturnRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostTestHelpersTreasuryInboundTransfersIdReturnRequest> {
    type Output = httpclient::InMemoryResult<TreasuryInboundTransfer>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/test_helpers/treasury/inbound_transfers/{id}/return", id = self
                .params.id
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}