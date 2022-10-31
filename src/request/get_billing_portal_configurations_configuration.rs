use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetBillingPortalConfigurationsConfigurationRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub configuration: String,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetBillingPortalConfigurationsConfigurationRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BillingPortalConfiguration> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/v1/billing_portal/configurations/{configuration}", configuration =
                    self.configuration
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
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
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}