use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetProductsRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub active: Option<bool>,
    pub created: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub ids: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub shippable: Option<bool>,
    pub starting_after: Option<String>,
    pub url: Option<String>,
}
impl<'a> GetProductsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/v1/products");
        if let Some(ref unwrapped) = self.active {
            r = r.push_query("active", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created {
            r = r.push_query("created", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.push_query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.ids {
            for item in unwrapped {
                r = r.push_query("ids[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.push_query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.shippable {
            r = r.push_query("shippable", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.push_query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.url {
            r = r.push_query("url", &unwrapped.to_string());
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
    pub fn created(mut self, created: serde_json::Value) -> Self {
        self.created = Some(created);
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
    pub fn ids(mut self, ids: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.ids = Some(ids.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn shippable(mut self, shippable: bool) -> Self {
        self.shippable = Some(shippable);
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.starting_after = Some(starting_after.to_owned());
        self
    }
    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_owned());
        self
    }
}
