use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_test_helpers_treasury_received_credits`].

On request success, this will return a [`TreasuryReceivedCredit`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTestHelpersTreasuryReceivedCreditsRequest {}
impl PostTestHelpersTreasuryReceivedCreditsRequest {}
impl FluentRequest<'_, PostTestHelpersTreasuryReceivedCreditsRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostTestHelpersTreasuryReceivedCreditsRequest> {
    type Output = httpclient::InMemoryResult<TreasuryReceivedCredit>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/test_helpers/treasury/received_credits";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}