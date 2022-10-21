use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PostIssuingCardsRequest<'a> {
    pub(crate) client: &'a StripeClient,
}
impl<'a> PostIssuingCardsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<IssuingCard> {
        let mut r = self.client.client.post("/v1/issuing/cards");
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
