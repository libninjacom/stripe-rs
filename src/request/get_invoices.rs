use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInvoicesRequest {
    pub collection_method: Option<String>,
    pub created: Option<serde_json::Value>,
    pub customer: Option<String>,
    pub due_date: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
    pub subscription: Option<String>,
}
impl GetInvoicesRequest {}
impl FluentRequest<'_, GetInvoicesRequest> {
    pub fn collection_method(mut self, collection_method: &str) -> Self {
        self.params.collection_method = Some(collection_method.to_owned());
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.params.created = Some(created);
        self
    }
    pub fn customer(mut self, customer: &str) -> Self {
        self.params.customer = Some(customer.to_owned());
        self
    }
    pub fn due_date(mut self, due_date: serde_json::Value) -> Self {
        self.params.due_date = Some(due_date);
        self
    }
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
    pub fn status(mut self, status: &str) -> Self {
        self.params.status = Some(status.to_owned());
        self
    }
    pub fn subscription(mut self, subscription: &str) -> Self {
        self.params.subscription = Some(subscription.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetInvoicesRequest> {
    type Output = httpclient::InMemoryResult<InvoicesList>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/v1/invoices";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}