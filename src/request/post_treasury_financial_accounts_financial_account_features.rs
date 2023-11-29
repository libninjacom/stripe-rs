use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountFeaturesRequest {
    pub financial_account: String,
}
impl PostTreasuryFinancialAccountsFinancialAccountFeaturesRequest {}
impl FluentRequest<'_, PostTreasuryFinancialAccountsFinancialAccountFeaturesRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostTreasuryFinancialAccountsFinancialAccountFeaturesRequest> {
    type Output = httpclient::InMemoryResult<TreasuryFinancialAccountFeatures>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = &format!(
                "/v1/treasury/financial_accounts/{financial_account}/features",
                financial_account = self.params.financial_account
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}