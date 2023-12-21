use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_customers_customer_balance_transactions_transaction`].

On request success, this will return a [`CustomerBalanceTransaction`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostCustomersCustomerBalanceTransactionsTransactionRequest {
    pub customer: String,
    pub transaction: String,
}
impl PostCustomersCustomerBalanceTransactionsTransactionRequest {}
impl FluentRequest<'_, PostCustomersCustomerBalanceTransactionsTransactionRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostCustomersCustomerBalanceTransactionsTransactionRequest> {
    type Output = httpclient::InMemoryResult<CustomerBalanceTransaction>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/customers/{customer}/balance_transactions/{transaction}", customer =
                self.params.customer, transaction = self.params.transaction
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}