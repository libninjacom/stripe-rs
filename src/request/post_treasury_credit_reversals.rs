use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostTreasuryCreditReversalsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostTreasuryCreditReversalsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TreasuryCreditReversal> {
        let mut r = self.client.client.post("/v1/treasury/credit_reversals");
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