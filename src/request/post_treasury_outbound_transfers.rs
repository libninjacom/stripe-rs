use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_treasury_outbound_transfers`].

On request success, this will return a [`TreasuryOutboundTransfer`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTreasuryOutboundTransfersRequest {}
impl PostTreasuryOutboundTransfersRequest {}
impl FluentRequest<'_, PostTreasuryOutboundTransfersRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostTreasuryOutboundTransfersRequest> {
    type Output = httpclient::InMemoryResult<TreasuryOutboundTransfer>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/treasury/outbound_transfers";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}