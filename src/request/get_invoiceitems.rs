use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetInvoiceitemsRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub created: Option<serde_json::Value>,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub invoice: Option<String>,
    pub limit: Option<i64>,
    pub pending: Option<bool>,
    pub starting_after: Option<String>,
}
impl<'a> GetInvoiceitemsRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<InvoicesItemsList> {
        let mut r = self.http_client.client.get("/v1/invoiceitems");
        if let Some(ref unwrapped) = self.created {
            r = r.query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.customer {
            r = r.query("customer", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.invoice {
            r = r.query("invoice", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.pending {
            r = r.query("pending", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.query("starting_after", &unwrapped.to_string());
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn customer(mut self, customer: &str) -> Self {
        self.customer = Some(customer.to_owned());
        self
    }
    pub fn ending_before(mut self, ending_before: &str) -> Self {
        self.ending_before = Some(ending_before.to_owned());
        self
    }
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn invoice(mut self, invoice: &str) -> Self {
        self.invoice = Some(invoice.to_owned());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn pending(mut self, pending: bool) -> Self {
        self.pending = Some(pending);
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.starting_after = Some(starting_after.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for GetInvoiceitemsRequest<'a> {
    type Output = httpclient::InMemoryResult<InvoicesItemsList>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}