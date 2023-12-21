use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_treasury_outbound_payments`].

On request success, this will return a [`TreasuryOutboundPayment`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTreasuryOutboundPaymentsRequest {}
impl PostTreasuryOutboundPaymentsRequest {}
impl FluentRequest<'_, PostTreasuryOutboundPaymentsRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostTreasuryOutboundPaymentsRequest> {
    type Output = httpclient::InMemoryResult<TreasuryOutboundPayment>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/treasury/outbound_payments";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}