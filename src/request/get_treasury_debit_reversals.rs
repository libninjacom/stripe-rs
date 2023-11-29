use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTreasuryDebitReversalsRequest {
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub financial_account: String,
    pub limit: Option<i64>,
    pub received_debit: Option<String>,
    pub resolution: Option<String>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
}
impl GetTreasuryDebitReversalsRequest {}
impl FluentRequest<'_, GetTreasuryDebitReversalsRequest> {
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
    pub fn received_debit(mut self, received_debit: &str) -> Self {
        self.params.received_debit = Some(received_debit.to_owned());
        self
    }
    pub fn resolution(mut self, resolution: &str) -> Self {
        self.params.resolution = Some(resolution.to_owned());
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.params.starting_after = Some(starting_after.to_owned());
        self
    }
    pub fn status(mut self, status: &str) -> Self {
        self.params.status = Some(status.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetTreasuryDebitReversalsRequest> {
    type Output = httpclient::InMemoryResult<
        TreasuryReceivedDebitsResourceDebitReversalList,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/v1/treasury/debit_reversals";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}