use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetTreasuryTransactionEntriesRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
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
impl<'a> GetTreasuryTransactionEntriesRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<TreasuryTransactionsResourceTransactionEntryList> {
        let mut r = self.http_client.client.get("/v1/treasury/transaction_entries");
        if let Some(ref unwrapped) = self.created {
            r = r.query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.effective_at {
            r = r.query("effective_at", &unwrapped.to_string());
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
        if let Some(ref unwrapped) = self.transaction {
            r = r.query("transaction", &unwrapped.to_string());
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn effective_at(mut self, effective_at: serde_json::Value) -> Self {
        self.effective_at = Some(effective_at);
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
    pub fn transaction(mut self, transaction: &str) -> Self {
        self.transaction = Some(transaction.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for GetTreasuryTransactionEntriesRequest<'a> {
    type Output = httpclient::InMemoryResult<
        TreasuryTransactionsResourceTransactionEntryList,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}