use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_financial_connections_accounts`].

On request success, this will return a [`GetFinancialConnectionsAccountsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFinancialConnectionsAccountsRequest {
    pub account_holder: Option<AccountholderParams>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub session: Option<String>,
    pub starting_after: Option<String>,
}
impl GetFinancialConnectionsAccountsRequest {}
impl FluentRequest<'_, GetFinancialConnectionsAccountsRequest> {
    pub fn account_holder(mut self, account_holder: AccountholderParams) -> Self {
        self.params.account_holder = Some(account_holder);
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
    pub fn session(mut self, session: &str) -> Self {
        self.params.session = Some(session.to_owned());
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.params.starting_after = Some(starting_after.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetFinancialConnectionsAccountsRequest> {
    type Output = httpclient::InMemoryResult<GetFinancialConnectionsAccountsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/financial_connections/accounts";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}