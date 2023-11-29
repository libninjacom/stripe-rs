use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetCreditNotesPreviewRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub amount: Option<i64>,
    pub credit_amount: Option<i64>,
    pub effective_at: Option<i64>,
    pub expand: Option<Vec<String>>,
    pub invoice: String,
    pub lines: Option<Vec<CreditNoteLineItemParams>>,
    pub memo: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub out_of_band_amount: Option<i64>,
    pub reason: Option<String>,
    pub refund: Option<String>,
    pub refund_amount: Option<i64>,
    pub shipping_cost: Option<CreditNoteShippingCost>,
}
impl<'a> GetCreditNotesPreviewRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<CreditNote> {
        let mut r = self.http_client.client.get("/v1/credit_notes/preview");
        if let Some(ref unwrapped) = self.amount {
            r = r.query("amount", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.credit_amount {
            r = r.query("credit_amount", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.effective_at {
            r = r.query("effective_at", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.query("expand[]", &item.to_string());
            }
        }
        r = r.query("invoice", &self.invoice.to_string());
        if let Some(ref unwrapped) = self.lines {
            for item in unwrapped {
                r = r.query("lines[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.memo {
            r = r.query("memo", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.metadata {
            r = r.query("metadata", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.out_of_band_amount {
            r = r.query("out_of_band_amount", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.reason {
            r = r.query("reason", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.refund {
            r = r.query("refund", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.refund_amount {
            r = r.query("refund_amount", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.shipping_cost {
            r = r.query("shipping_cost", &unwrapped.to_string());
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn amount(mut self, amount: i64) -> Self {
        self.amount = Some(amount);
        self
    }
    pub fn credit_amount(mut self, credit_amount: i64) -> Self {
        self.credit_amount = Some(credit_amount);
        self
    }
    pub fn effective_at(mut self, effective_at: i64) -> Self {
        self.effective_at = Some(effective_at);
        self
    }
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn lines(mut self, lines: Vec<CreditNoteLineItemParams>) -> Self {
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
    pub fn shipping_cost(mut self, shipping_cost: CreditNoteShippingCost) -> Self {
        self.shipping_cost = Some(shipping_cost);
        self
    }
}
impl<'a> ::std::future::IntoFuture for GetCreditNotesPreviewRequest<'a> {
    type Output = httpclient::InMemoryResult<CreditNote>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}