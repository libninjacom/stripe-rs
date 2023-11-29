use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostTestHelpersIssuingTransactionsTransactionRefundRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub transaction: String,
}
impl<'a> PostTestHelpersIssuingTransactionsTransactionRefundRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<IssuingTransaction> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v1/test_helpers/issuing/transactions/{transaction}/refund",
                    transaction = self.transaction
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture
for PostTestHelpersIssuingTransactionsTransactionRefundRequest<'a> {
    type Output = httpclient::InMemoryResult<IssuingTransaction>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}