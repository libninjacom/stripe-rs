use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetBillingPortalConfigurationsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub active: Option<bool>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub is_default: Option<bool>,
    pub limit: Option<i64>,
    pub starting_after: Option<String>,
}
impl<'a> GetBillingPortalConfigurationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/billing_portal/configurations");
        if let Some(ref unwrapped) = self.active {
            r = r.push_query("active", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.is_default {
            r = r.push_query("is_default", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
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
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
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
    pub fn is_default(mut self, is_default: bool) -> Self {
        self.is_default = Some(is_default);
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
