use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_test_helpers_customers_customer_fund_cash_balance`].

On request success, this will return a [`CustomerCashBalanceTransaction`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTestHelpersCustomersCustomerFundCashBalanceRequest {
    pub customer: String,
}
impl PostTestHelpersCustomersCustomerFundCashBalanceRequest {}
impl FluentRequest<'_, PostTestHelpersCustomersCustomerFundCashBalanceRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostTestHelpersCustomersCustomerFundCashBalanceRequest> {
    type Output = httpclient::InMemoryResult<CustomerCashBalanceTransaction>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/test_helpers/customers/{customer}/fund_cash_balance", customer =
                self.params.customer
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}