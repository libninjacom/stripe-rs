use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCustomersCustomerPaymentMethodsRequest {
    pub customer: String,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub type_: Option<String>,
}
impl GetCustomersCustomerPaymentMethodsRequest {}
impl FluentRequest<'_, GetCustomersCustomerPaymentMethodsRequest> {
    pub fn ending_before(mut self, ending_before: &str) -> Self {
        self.params.ending_before = Some(ending_before.to_owned());
        self
    }
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.params.limit = Some(limit);
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.params.starting_after = Some(starting_after.to_owned());
        self
    }
    pub fn type_(mut self, type_: &str) -> Self {
        self.params.type_ = Some(type_.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetCustomersCustomerPaymentMethodsRequest> {
    type Output = httpclient::InMemoryResult<CustomerPaymentMethodResourceList>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = &format!(
                "/v1/customers/{customer}/payment_methods", customer = self.params
                .customer
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}