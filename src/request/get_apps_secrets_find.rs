use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetAppsSecretsFindRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub name: String,
    pub scope: serde_json::Value,
}
impl<'a> GetAppsSecretsFindRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AppsSecret> {
        let mut r = self.client.client.get("/v1/apps/secrets/find");
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = r.push_query("name", &self.name.to_string());
        r = r.push_query("scope", &self.scope.to_string());
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
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
