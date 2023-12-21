use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_financial_connections_transactions`].

On request success, this will return a [`GetFinancialConnectionsTransactionsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFinancialConnectionsTransactionsRequest {
    pub account: String,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub transacted_at: Option<serde_json::Value>,
    pub transaction_refresh: Option<TransactionRefreshParams>,
}
impl GetFinancialConnectionsTransactionsRequest {}
impl FluentRequest<'_, GetFinancialConnectionsTransactionsRequest> {
    pub fn ending_before(mut self, ending_before: &str) -> Self {
        self.params.ending_before = Some(ending_before.to_owned());
        self
    }
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.params.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.params.starting_after = Some(starting_after.to_owned());
        self
    }
    pub fn transacted_at(mut self, transacted_at: serde_json::Value) -> Self {
        self.params.transacted_at = Some(transacted_at);
        self
    }
    pub fn transaction_refresh(
        mut self,
        transaction_refresh: TransactionRefreshParams,
    ) -> Self {
        self.params.transaction_refresh = Some(transaction_refresh);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetFinancialConnectionsTransactionsRequest> {
    type Output = httpclient::InMemoryResult<
        GetFinancialConnectionsTransactionsResponse,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/financial_connections/transactions";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}