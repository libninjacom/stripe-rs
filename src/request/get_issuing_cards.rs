use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetIssuingCardsRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub cardholder: Option<String>,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub exp_month: Option<i64>,
    pub exp_year: Option<i64>,
    pub expand: Option<Vec<String>>,
    pub last4: Option<String>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
    pub status: Option<String>,
    pub type_: Option<String>,
}
impl<'a> GetIssuingCardsRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<IssuingCardList> {
        let mut r = self.http_client.client.get("/v1/issuing/cards");
        if let Some(ref unwrapped) = self.cardholder {
            r = r.query("cardholder", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created {
            r = r.query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.exp_month {
            r = r.query("exp_month", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.exp_year {
            r = r.query("exp_year", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.last4 {
            r = r.query("last4", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.query("status", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.type_ {
            r = r.query("type", &unwrapped.to_string());
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn cardholder(mut self, cardholder: &str) -> Self {
        self.cardholder = Some(cardholder.to_owned());
        self
    }
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
        self
    }
    pub fn ending_before(mut self, ending_before: &str) -> Self {
        self.ending_before = Some(ending_before.to_owned());
        self
    }
    pub fn exp_month(mut self, exp_month: i64) -> Self {
        self.exp_month = Some(exp_month);
        self
    }
    pub fn exp_year(mut self, exp_year: i64) -> Self {
        self.exp_year = Some(exp_year);
        self
    }
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn last4(mut self, last4: &str) -> Self {
        self.last4 = Some(last4.to_owned());
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
    pub fn status(mut self, status: &str) -> Self {
        self.status = Some(status.to_owned());
        self
    }
    pub fn type_(mut self, type_: &str) -> Self {
        self.type_ = Some(type_.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for GetIssuingCardsRequest<'a> {
    type Output = httpclient::InMemoryResult<IssuingCardList>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}