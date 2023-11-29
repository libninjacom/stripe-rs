use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostTestHelpersCustomersCustomerFundCashBalanceRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub customer: String,
}
impl<'a> PostTestHelpersCustomersCustomerFundCashBalanceRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<CustomerCashBalanceTransaction> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v1/test_helpers/customers/{customer}/fund_cash_balance", customer =
                    self.customer
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture
for PostTestHelpersCustomersCustomerFundCashBalanceRequest<'a> {
    type Output = httpclient::InMemoryResult<CustomerCashBalanceTransaction>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}