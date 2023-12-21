use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_accounts_account_people_person`].

On request success, this will return a [`Person`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccountsAccountPeoplePersonRequest {
    pub account: String,
    pub expand: Option<Vec<String>>,
    pub person: String,
}
impl GetAccountsAccountPeoplePersonRequest {}
impl FluentRequest<'_, GetAccountsAccountPeoplePersonRequest> {
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetAccountsAccountPeoplePersonRequest> {
    type Output = httpclient::InMemoryResult<Person>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/accounts/{account}/people/{person}", account = self.params.account,
                person = self.params.person
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}