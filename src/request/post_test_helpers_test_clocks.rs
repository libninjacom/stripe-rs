use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostTestHelpersTestClocksRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostTestHelpersTestClocksRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TestHelpersTestClock> {
        let mut r = self.client.client.post("/v1/test_helpers/test_clocks");
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
