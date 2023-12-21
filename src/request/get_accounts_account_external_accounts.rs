use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_accounts_account_external_accounts`].

On request success, this will return a [`GetAccountsAccountExternalAccountsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccountsAccountExternalAccountsRequest {
    pub account: String,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub object: Option<String>,
    pub starting_after: Option<String>,
}
impl GetAccountsAccountExternalAccountsRequest {}
impl FluentRequest<'_, GetAccountsAccountExternalAccountsRequest> {
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
    pub fn object(mut self, object: &str) -> Self {
        self.params.object = Some(object.to_owned());
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.params.starting_after = Some(starting_after.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetAccountsAccountExternalAccountsRequest> {
    type Output = httpclient::InMemoryResult<GetAccountsAccountExternalAccountsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/accounts/{account}/external_accounts", account = self.params.account
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}