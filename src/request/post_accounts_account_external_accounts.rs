use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_accounts_account_external_accounts`].

On request success, this will return a [`ExternalAccount`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostAccountsAccountExternalAccountsRequest {
    pub account: String,
}
impl PostAccountsAccountExternalAccountsRequest {}
impl FluentRequest<'_, PostAccountsAccountExternalAccountsRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostAccountsAccountExternalAccountsRequest> {
    type Output = httpclient::InMemoryResult<ExternalAccount>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/accounts/{account}/external_accounts", account = self.params.account
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}