use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_customers_customer_payment_methods_payment_method`].

On request success, this will return a [`PaymentMethod`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCustomersCustomerPaymentMethodsPaymentMethodRequest {
    pub customer: String,
    pub expand: Option<Vec<String>>,
    pub payment_method: String,
}
impl GetCustomersCustomerPaymentMethodsPaymentMethodRequest {}
impl FluentRequest<'_, GetCustomersCustomerPaymentMethodsPaymentMethodRequest> {
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetCustomersCustomerPaymentMethodsPaymentMethodRequest> {
    type Output = httpclient::InMemoryResult<PaymentMethod>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/customers/{customer}/payment_methods/{payment_method}", customer =
                self.params.customer, payment_method = self.params.payment_method
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}