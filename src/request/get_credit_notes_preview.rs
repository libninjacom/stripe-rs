use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_credit_notes_preview`].

On request success, this will return a [`CreditNote`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCreditNotesPreviewRequest {
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
impl GetCreditNotesPreviewRequest {}
impl FluentRequest<'_, GetCreditNotesPreviewRequest> {
    pub fn amount(mut self, amount: i64) -> Self {
        self.params.amount = Some(amount);
        self
    }
    pub fn credit_amount(mut self, credit_amount: i64) -> Self {
        self.params.credit_amount = Some(credit_amount);
        self
    }
    pub fn effective_at(mut self, effective_at: i64) -> Self {
        self.params.effective_at = Some(effective_at);
        self
    }
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn lines(mut self, lines: Vec<CreditNoteLineItemParams>) -> Self {
        self.params.lines = Some(lines);
        self
    }
    pub fn memo(mut self, memo: &str) -> Self {
        self.params.memo = Some(memo.to_owned());
        self
    }
    pub fn metadata(mut self, metadata: serde_json::Value) -> Self {
        self.params.metadata = Some(metadata);
        self
    }
    pub fn out_of_band_amount(mut self, out_of_band_amount: i64) -> Self {
        self.params.out_of_band_amount = Some(out_of_band_amount);
        self
    }
    pub fn reason(mut self, reason: &str) -> Self {
        self.params.reason = Some(reason.to_owned());
        self
    }
    pub fn refund(mut self, refund: &str) -> Self {
        self.params.refund = Some(refund.to_owned());
        self
    }
    pub fn refund_amount(mut self, refund_amount: i64) -> Self {
        self.params.refund_amount = Some(refund_amount);
        self
    }
    pub fn shipping_cost(mut self, shipping_cost: CreditNoteShippingCost) -> Self {
        self.params.shipping_cost = Some(shipping_cost);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetCreditNotesPreviewRequest> {
    type Output = httpclient::InMemoryResult<CreditNote>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/credit_notes/preview";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}