use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetPromotionCodesRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub active: Option<bool>,
    pub code: Option<String>,
    pub coupon: Option<String>,
    pub created: Option<serde_json::Value>,
    pub customer: Option<String>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetPromotionCodesRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<PromotionCodesResourcePromotionCodeList> {
        let mut r = self.http_client.client.get("/v1/promotion_codes");
        if let Some(ref unwrapped) = self.active {
            r = r.query("active", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.code {
            r = r.query("code", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.coupon {
            r = r.query("coupon", &unwrapped.to_string());
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
        if let Some(ref unwrapped) = self.starting_after {
            r = r.query("starting_after", &unwrapped.to_string());
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }
    pub fn code(mut self, code: &str) -> Self {
        self.code = Some(code.to_owned());
        self
    }
    pub fn coupon(mut self, coupon: &str) -> Self {
        self.coupon = Some(coupon.to_owned());
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
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.starting_after = Some(starting_after.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for GetPromotionCodesRequest<'a> {
    type Output = httpclient::InMemoryResult<PromotionCodesResourcePromotionCodeList>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}