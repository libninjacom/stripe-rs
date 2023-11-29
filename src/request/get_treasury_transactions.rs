use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetTreasuryTransactionsRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub financial_account: String,
    pub limit: Option<i64>,
    pub order_by: Option<String>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
    pub status_transitions: Option<StatusTransitionTimestampSpecs>,
}
impl<'a> GetTreasuryTransactionsRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<TreasuryTransactionsResourceTransactionList> {
        let mut r = self.http_client.client.get("/v1/treasury/transactions");
        if let Some(ref unwrapped) = self.created {
            r = r.query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.query("expand[]", &item.to_string());
            }
        }
        r = r.query("financial_account", &self.financial_account.to_string());
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.order_by {
            r = r.query("order_by", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.query("status", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status_transitions {
            r = r.query("status_transitions", &unwrapped.to_string());
        }
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: &str) -> Self {
        self.ending_before = Some(ending_before.to_owned());
        self
    }
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn order_by(mut self, order_by: &str) -> Self {
        self.order_by = Some(order_by.to_owned());
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.starting_after = Some(starting_after.to_owned());
        self
    }
    pub fn status(mut self, status: &str) -> Self {
        self.status = Some(status.to_owned());
        self
    }
    pub fn status_transitions(
        mut self,
        status_transitions: StatusTransitionTimestampSpecs,
    ) -> Self {
        self.status_transitions = Some(status_transitions);
        self
    }
}
impl<'a> ::std::future::IntoFuture for GetTreasuryTransactionsRequest<'a> {
    type Output = httpclient::InMemoryResult<
        TreasuryTransactionsResourceTransactionList,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}