use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_sources_source_source_transactions_source_transaction`].

On request success, this will return a [`SourceTransaction`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSourcesSourceSourceTransactionsSourceTransactionRequest {
    pub expand: Option<Vec<String>>,
    pub source: String,
    pub source_transaction: String,
}
impl GetSourcesSourceSourceTransactionsSourceTransactionRequest {}
impl FluentRequest<'_, GetSourcesSourceSourceTransactionsSourceTransactionRequest> {
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetSourcesSourceSourceTransactionsSourceTransactionRequest> {
    type Output = httpclient::InMemoryResult<SourceTransaction>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/sources/{source}/source_transactions/{source_transaction}", source =
                self.params.source, source_transaction = self.params.source_transaction
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}