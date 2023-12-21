use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_issuing_transactions_transaction`].

On request success, this will return a [`IssuingTransaction`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostIssuingTransactionsTransactionRequest {
    pub transaction: String,
}
impl PostIssuingTransactionsTransactionRequest {}
impl FluentRequest<'_, PostIssuingTransactionsTransactionRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostIssuingTransactionsTransactionRequest> {
    type Output = httpclient::InMemoryResult<IssuingTransaction>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/issuing/transactions/{transaction}", transaction = self.params
                .transaction
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}