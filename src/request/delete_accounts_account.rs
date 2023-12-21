use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::delete_accounts_account`].

On request success, this will return a [`DeletedAccount`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAccountsAccountRequest {
    pub account: String,
}
impl DeleteAccountsAccountRequest {}
impl FluentRequest<'_, DeleteAccountsAccountRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, DeleteAccountsAccountRequest> {
    type Output = httpclient::InMemoryResult<DeletedAccount>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/v1/accounts/{account}", account = self.params.account);
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}