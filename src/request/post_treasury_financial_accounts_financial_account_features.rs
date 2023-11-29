use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostTreasuryFinancialAccountsFinancialAccountFeaturesRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub financial_account: String,
}
impl<'a> PostTreasuryFinancialAccountsFinancialAccountFeaturesRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<TreasuryFinancialAccountFeatures> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v1/treasury/financial_accounts/{financial_account}/features",
                    financial_account = self.financial_account
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture
for PostTreasuryFinancialAccountsFinancialAccountFeaturesRequest<'a> {
    type Output = httpclient::InMemoryResult<TreasuryFinancialAccountFeatures>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}