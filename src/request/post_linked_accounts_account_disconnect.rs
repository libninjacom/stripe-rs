use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_linked_accounts_account_disconnect`].

On request success, this will return a [`FinancialConnectionsAccount`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostLinkedAccountsAccountDisconnectRequest {
    pub account: String,
}
impl PostLinkedAccountsAccountDisconnectRequest {}
impl FluentRequest<'_, PostLinkedAccountsAccountDisconnectRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostLinkedAccountsAccountDisconnectRequest> {
    type Output = httpclient::InMemoryResult<FinancialConnectionsAccount>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/linked_accounts/{account}/disconnect", account = self.params.account
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}