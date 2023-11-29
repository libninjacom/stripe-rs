use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTreasuryTransactionEntriesRequest {
    pub created: Option<serde_json::Value>,
    pub effective_at: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub financial_account: String,
    pub limit: Option<i64>,
    pub order_by: Option<String>,
    pub starting_after: Option<String>,
    pub transaction: Option<String>,
}
impl GetTreasuryTransactionEntriesRequest {}
impl FluentRequest<'_, GetTreasuryTransactionEntriesRequest> {
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.params.created = Some(created);
        self
    }
    pub fn effective_at(mut self, effective_at: serde_json::Value) -> Self {
        self.params.effective_at = Some(effective_at);
        self
    }
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
    pub fn order_by(mut self, order_by: &str) -> Self {
        self.params.order_by = Some(order_by.to_owned());
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.params.starting_after = Some(starting_after.to_owned());
        self
    }
    pub fn transaction(mut self, transaction: &str) -> Self {
        self.params.transaction = Some(transaction.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetTreasuryTransactionEntriesRequest> {
    type Output = httpclient::InMemoryResult<
        TreasuryTransactionsResourceTransactionEntryList,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/v1/treasury/transaction_entries";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}