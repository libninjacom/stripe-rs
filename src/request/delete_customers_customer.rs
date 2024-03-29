use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::delete_customers_customer`].

On request success, this will return a [`DeletedCustomer`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCustomersCustomerRequest {
    pub customer: String,
}
impl DeleteCustomersCustomerRequest {}
impl FluentRequest<'_, DeleteCustomersCustomerRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, DeleteCustomersCustomerRequest> {
    type Output = httpclient::InMemoryResult<DeletedCustomer>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/customers/{customer}", customer = self.params.customer
            );
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}