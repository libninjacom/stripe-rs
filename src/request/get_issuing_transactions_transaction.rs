use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_issuing_transactions_transaction`].

On request success, this will return a [`IssuingTransaction`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetIssuingTransactionsTransactionRequest {
    pub expand: Option<Vec<String>>,
    pub transaction: String,
}
impl GetIssuingTransactionsTransactionRequest {}
impl FluentRequest<'_, GetIssuingTransactionsTransactionRequest> {
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetIssuingTransactionsTransactionRequest> {
    type Output = httpclient::InMemoryResult<IssuingTransaction>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/issuing/transactions/{transaction}", transaction = self.params
                .transaction
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}