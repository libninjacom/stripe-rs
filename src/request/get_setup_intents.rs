use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetSetupIntentsRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub attach_to_self: Option<bool>,
    pub created: Option<serde_json::Value>,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payment_method: Option<String>,
    pub starting_after: Option<String>,
}
impl<'a> GetSetupIntentsRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<PaymentFlowsSetupIntentList> {
        let mut r = self.http_client.client.get("/v1/setup_intents");
        if let Some(ref unwrapped) = self.attach_to_self {
            r = r.query("attach_to_self", &unwrapped.to_string());
        }
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
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.payment_method {
            r = r.query("payment_method", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.query("starting_after", &unwrapped.to_string());
        }
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
    pub fn attach_to_self(mut self, attach_to_self: bool) -> Self {
        self.attach_to_self = Some(attach_to_self);
        self
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
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn payment_method(mut self, payment_method: &str) -> Self {
        self.payment_method = Some(payment_method.to_owned());
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.starting_after = Some(starting_after.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for GetSetupIntentsRequest<'a> {
    type Output = httpclient::InMemoryResult<PaymentFlowsSetupIntentList>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}