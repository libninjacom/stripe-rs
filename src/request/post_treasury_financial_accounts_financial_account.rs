use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostTreasuryFinancialAccountsFinancialAccountRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub financial_account: String,
}
impl<'a> PostTreasuryFinancialAccountsFinancialAccountRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<TreasuryFinancialAccount> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v1/treasury/financial_accounts/{financial_account}",
                    financial_account = self.financial_account
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture
for PostTreasuryFinancialAccountsFinancialAccountRequest<'a> {
    type Output = httpclient::InMemoryResult<TreasuryFinancialAccount>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}