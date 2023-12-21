use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_financial_connections_accounts_account_unsubscribe`].

On request success, this will return a [`FinancialConnectionsAccount`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostFinancialConnectionsAccountsAccountUnsubscribeRequest {
    pub account: String,
}
impl PostFinancialConnectionsAccountsAccountUnsubscribeRequest {}
impl FluentRequest<'_, PostFinancialConnectionsAccountsAccountUnsubscribeRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostFinancialConnectionsAccountsAccountUnsubscribeRequest> {
    type Output = httpclient::InMemoryResult<FinancialConnectionsAccount>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/financial_connections/accounts/{account}/unsubscribe", account =
                self.params.account
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}