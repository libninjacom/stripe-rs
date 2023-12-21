use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::delete_customers_customer_discount`].

On request success, this will return a [`DeletedDiscount`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCustomersCustomerDiscountRequest {
    pub customer: String,
}
impl DeleteCustomersCustomerDiscountRequest {}
impl FluentRequest<'_, DeleteCustomersCustomerDiscountRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, DeleteCustomersCustomerDiscountRequest> {
    type Output = httpclient::InMemoryResult<DeletedDiscount>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/customers/{customer}/discount", customer = self.params.customer
            );
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}