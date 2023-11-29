use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetBalanceHistoryRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub currency: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payout: Option<String>,
    pub source: Option<String>,
    pub starting_after: Option<String>,
    pub type_: Option<String>,
}
impl<'a> GetBalanceHistoryRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<BalanceTransactionsList> {
        let mut r = self.http_client.client.get("/v1/balance/history");
        if let Some(ref unwrapped) = self.created {
            r = r.query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.currency {
            r = r.query("currency", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.payout {
            r = r.query("payout", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.source {
            r = r.query("source", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.type_ {
            r = r.query("type", &unwrapped.to_string());
        }
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn currency(mut self, currency: &str) -> Self {
        self.currency = Some(currency.to_owned());
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
    pub fn payout(mut self, payout: &str) -> Self {
        self.payout = Some(payout.to_owned());
        self
    }
    pub fn source(mut self, source: &str) -> Self {
        self.source = Some(source.to_owned());
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.starting_after = Some(starting_after.to_owned());
        self
    }
    pub fn type_(mut self, type_: &str) -> Self {
        self.type_ = Some(type_.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for GetBalanceHistoryRequest<'a> {
    type Output = httpclient::InMemoryResult<BalanceTransactionsList>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}