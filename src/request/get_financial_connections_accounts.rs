use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetFinancialConnectionsAccountsRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub account_holder: Option<AccountholderParams>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub session: Option<String>,
    pub starting_after: Option<String>,
}
impl<'a> GetFinancialConnectionsAccountsRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<BankConnectionsResourceLinkedAccountList> {
        let mut r = self.http_client.client.get("/v1/financial_connections/accounts");
        if let Some(ref unwrapped) = self.account_holder {
            r = r.query("account_holder", &unwrapped.to_string());
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
        if let Some(ref unwrapped) = self.session {
            r = r.query("session", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.query("starting_after", &unwrapped.to_string());
        }
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
    pub fn account_holder(mut self, account_holder: AccountholderParams) -> Self {
        self.account_holder = Some(account_holder);
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
    pub fn session(mut self, session: &str) -> Self {
        self.session = Some(session.to_owned());
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.starting_after = Some(starting_after.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for GetFinancialConnectionsAccountsRequest<'a> {
    type Output = httpclient::InMemoryResult<BankConnectionsResourceLinkedAccountList>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}