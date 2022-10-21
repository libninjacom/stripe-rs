use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetIssuingCardsRequest<'a> {
    pub(crate) client: &'a StripeClient,
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
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/issuing/cards");
        if let Some(ref unwrapped) = self.cardholder {
            r = r.push_query("cardholder", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.exp_month {
            r = r.push_query("exp_month", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.exp_year {
            r = r.push_query("exp_year", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.last4 {
            r = r.push_query("last4", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.push_query("status", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.type_ {
            r = r.push_query("type", &unwrapped.to_string());
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
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
