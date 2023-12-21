use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_test_helpers_treasury_received_debits`].

On request success, this will return a [`TreasuryReceivedDebit`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTestHelpersTreasuryReceivedDebitsRequest {}
impl PostTestHelpersTreasuryReceivedDebitsRequest {}
impl FluentRequest<'_, PostTestHelpersTreasuryReceivedDebitsRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostTestHelpersTreasuryReceivedDebitsRequest> {
    type Output = httpclient::InMemoryResult<TreasuryReceivedDebit>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/test_helpers/treasury/received_debits";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}