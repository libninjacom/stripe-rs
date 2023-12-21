use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_accounts_account_people`].

On request success, this will return a [`GetAccountsAccountPeopleResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccountsAccountPeopleRequest {
    pub account: String,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub relationship: Option<AllPeopleRelationshipSpecs>,
    pub starting_after: Option<String>,
}
impl GetAccountsAccountPeopleRequest {}
impl FluentRequest<'_, GetAccountsAccountPeopleRequest> {
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
    pub fn relationship(mut self, relationship: AllPeopleRelationshipSpecs) -> Self {
        self.params.relationship = Some(relationship);
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.params.starting_after = Some(starting_after.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetAccountsAccountPeopleRequest> {
    type Output = httpclient::InMemoryResult<GetAccountsAccountPeopleResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/accounts/{account}/people", account = self.params.account
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}