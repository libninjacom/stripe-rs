use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetCheckoutSessionsRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub customer: Option<String>,
    pub customer_details: Option<CustomerDetailsParams>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub payment_intent: Option<String>,
    pub payment_link: Option<String>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
    pub subscription: Option<String>,
}
impl<'a> GetCheckoutSessionsRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<PaymentPagesCheckoutSessionList> {
        let mut r = self.http_client.client.get("/v1/checkout/sessions");
        if let Some(ref unwrapped) = self.customer {
            r = r.query("customer", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.customer_details {
            r = r.query("customer_details", &unwrapped.to_string());
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
        if let Some(ref unwrapped) = self.payment_intent {
            r = r.query("payment_intent", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.payment_link {
            r = r.query("payment_link", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.query("status", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription {
            r = r.query("subscription", &unwrapped.to_string());
        }
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
    pub fn customer(mut self, customer: &str) -> Self {
        self.customer = Some(customer.to_owned());
        self
    }
    pub fn customer_details(mut self, customer_details: CustomerDetailsParams) -> Self {
        self.customer_details = Some(customer_details);
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
    pub fn payment_intent(mut self, payment_intent: &str) -> Self {
        self.payment_intent = Some(payment_intent.to_owned());
        self
    }
    pub fn payment_link(mut self, payment_link: &str) -> Self {
        self.payment_link = Some(payment_link.to_owned());
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.starting_after = Some(starting_after.to_owned());
        self
    }
    pub fn status(mut self, status: &str) -> Self {
        self.status = Some(status.to_owned());
        self
    }
    pub fn subscription(mut self, subscription: &str) -> Self {
        self.subscription = Some(subscription.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for GetCheckoutSessionsRequest<'a> {
    type Output = httpclient::InMemoryResult<PaymentPagesCheckoutSessionList>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}