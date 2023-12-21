use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_treasury_debit_reversals`].

On request success, this will return a [`TreasuryDebitReversal`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTreasuryDebitReversalsRequest {}
impl PostTreasuryDebitReversalsRequest {}
impl FluentRequest<'_, PostTreasuryDebitReversalsRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostTreasuryDebitReversalsRequest> {
    type Output = httpclient::InMemoryResult<TreasuryDebitReversal>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/treasury/debit_reversals";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}