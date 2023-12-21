use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::delete_accounts_account_persons_person`].

On request success, this will return a [`DeletedPerson`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAccountsAccountPersonsPersonRequest {
    pub account: String,
    pub person: String,
}
impl DeleteAccountsAccountPersonsPersonRequest {}
impl FluentRequest<'_, DeleteAccountsAccountPersonsPersonRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, DeleteAccountsAccountPersonsPersonRequest> {
    type Output = httpclient::InMemoryResult<DeletedPerson>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/accounts/{account}/persons/{person}", account = self.params.account,
                person = self.params.person
            );
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}