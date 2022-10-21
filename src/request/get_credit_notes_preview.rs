use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetCreditNotesPreviewRequest<'a> {
    pub(crate) client: &'a StripeClient,
    pub amount: Option<i64>,
    pub credit_amount: Option<i64>,
    pub expand: Option<Vec<String>>,
    pub invoice: String,
    pub lines: Option<Vec<serde_json::Value>>,
    pub memo: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub out_of_band_amount: Option<i64>,
    pub reason: Option<String>,
    pub refund: Option<String>,
    pub refund_amount: Option<i64>,
}
impl<'a> GetCreditNotesPreviewRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CreditNote> {
        let mut r = self.client.client.get("/v1/credit_notes/preview");
        if let Some(ref unwrapped) = self.amount {
            r = r.push_query("amount", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.credit_amount {
            r = r.push_query("credit_amount", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.push_query("expand[]", &item.to_string());
            }
        }
        r = r.push_query("invoice", &self.invoice.to_string());
        if let Some(ref unwrapped) = self.lines {
            for item in unwrapped {
                r = r.push_query("lines[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.memo {
            r = r.push_query("memo", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.metadata {
            r = r.push_query("metadata", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.out_of_band_amount {
            r = r.push_query("out_of_band_amount", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.reason {
            r = r.push_query("reason", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.refund {
            r = r.push_query("refund", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.refund_amount {
            r = r.push_query("refund_amount", &unwrapped.to_string());
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
    pub fn amount(mut self, amount: i64) -> Self {
        self.amount = Some(amount);
        self
    }
    pub fn credit_amount(mut self, credit_amount: i64) -> Self {
        self.credit_amount = Some(credit_amount);
        self
    }
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn lines(mut self, lines: Vec<serde_json::Value>) -> Self {
        self.lines = Some(lines);
        self
    }
    pub fn memo(mut self, memo: &str) -> Self {
        self.memo = Some(memo.to_owned());
        self
    }
    pub fn metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
    pub fn out_of_band_amount(mut self, out_of_band_amount: i64) -> Self {
        self.out_of_band_amount = Some(out_of_band_amount);
        self
    }
    pub fn reason(mut self, reason: &str) -> Self {
        self.reason = Some(reason.to_owned());
        self
    }
    pub fn refund(mut self, refund: &str) -> Self {
        self.refund = Some(refund.to_owned());
        self
    }
    pub fn refund_amount(mut self, refund_amount: i64) -> Self {
        self.refund_amount = Some(refund_amount);
        self
    }
}
