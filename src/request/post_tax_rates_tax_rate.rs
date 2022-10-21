use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostTaxRatesTaxRateRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub tax_rate: String,
}
impl<'a> PostTaxRatesTaxRateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TaxRate> {
        let mut r = self
            .client
            .client
            .post(&format!("/v1/tax_rates/{tax_rate}", tax_rate = self.tax_rate));
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
}
