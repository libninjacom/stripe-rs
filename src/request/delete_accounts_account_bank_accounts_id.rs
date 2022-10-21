use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteAccountsAccountBankAccountsIdRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub account: String,
    pub id: String,
}
impl<'a> DeleteAccountsAccountBankAccountsIdRequest<'a> {
    pub async fn send(self) -> anyhow::Result<DeletedExternalAccount> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/v1/accounts/{account}/bank_accounts/{id}", account = self.account,
                    id = self.id
                ),
            );
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
