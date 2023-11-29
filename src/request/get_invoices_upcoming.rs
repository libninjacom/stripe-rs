use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInvoicesUpcomingRequest {
    pub automatic_tax: Option<AutomaticTaxParam>,
    pub coupon: Option<String>,
    pub currency: Option<String>,
    pub customer: Option<String>,
    pub customer_details: Option<CustomerDetailsParam>,
    pub discounts: Option<serde_json::Value>,
    pub expand: Option<Vec<String>>,
    pub invoice_items: Option<Vec<InvoiceItemPreviewParams>>,
    pub schedule: Option<String>,
    pub subscription: Option<String>,
    pub subscription_billing_cycle_anchor: Option<serde_json::Value>,
    pub subscription_cancel_at: Option<serde_json::Value>,
    pub subscription_cancel_at_period_end: Option<bool>,
    pub subscription_cancel_now: Option<bool>,
    pub subscription_default_tax_rates: Option<serde_json::Value>,
    pub subscription_items: Option<Vec<SubscriptionItemUpdateParams>>,
    pub subscription_proration_behavior: Option<String>,
    pub subscription_proration_date: Option<i64>,
    pub subscription_resume_at: Option<String>,
    pub subscription_start_date: Option<i64>,
    pub subscription_trial_end: Option<serde_json::Value>,
    pub subscription_trial_from_plan: Option<bool>,
}
impl GetInvoicesUpcomingRequest {}
impl FluentRequest<'_, GetInvoicesUpcomingRequest> {
    pub fn automatic_tax(mut self, automatic_tax: AutomaticTaxParam) -> Self {
        self.params.automatic_tax = Some(automatic_tax);
        self
    }
    pub fn coupon(mut self, coupon: &str) -> Self {
        self.params.coupon = Some(coupon.to_owned());
        self
    }
    pub fn currency(mut self, currency: &str) -> Self {
        self.params.currency = Some(currency.to_owned());
        self
    }
    pub fn customer(mut self, customer: &str) -> Self {
        self.params.customer = Some(customer.to_owned());
        self
    }
    pub fn customer_details(mut self, customer_details: CustomerDetailsParam) -> Self {
        self.params.customer_details = Some(customer_details);
        self
    }
    pub fn discounts(mut self, discounts: serde_json::Value) -> Self {
        self.params.discounts = Some(discounts);
        self
    }
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn invoice_items(
        mut self,
        invoice_items: Vec<InvoiceItemPreviewParams>,
    ) -> Self {
        self.params.invoice_items = Some(invoice_items);
        self
    }
    pub fn schedule(mut self, schedule: &str) -> Self {
        self.params.schedule = Some(schedule.to_owned());
        self
    }
    pub fn subscription(mut self, subscription: &str) -> Self {
        self.params.subscription = Some(subscription.to_owned());
        self
    }
    pub fn subscription_billing_cycle_anchor(
        mut self,
        subscription_billing_cycle_anchor: serde_json::Value,
    ) -> Self {
        self
            .params
            .subscription_billing_cycle_anchor = Some(subscription_billing_cycle_anchor);
        self
    }
    pub fn subscription_cancel_at(
        mut self,
        subscription_cancel_at: serde_json::Value,
    ) -> Self {
        self.params.subscription_cancel_at = Some(subscription_cancel_at);
        self
    }
    pub fn subscription_cancel_at_period_end(
        mut self,
        subscription_cancel_at_period_end: bool,
    ) -> Self {
        self
            .params
            .subscription_cancel_at_period_end = Some(subscription_cancel_at_period_end);
        self
    }
    pub fn subscription_cancel_now(mut self, subscription_cancel_now: bool) -> Self {
        self.params.subscription_cancel_now = Some(subscription_cancel_now);
        self
    }
    pub fn subscription_default_tax_rates(
        mut self,
        subscription_default_tax_rates: serde_json::Value,
    ) -> Self {
        self
            .params
            .subscription_default_tax_rates = Some(subscription_default_tax_rates);
        self
    }
    pub fn subscription_items(
        mut self,
        subscription_items: Vec<SubscriptionItemUpdateParams>,
    ) -> Self {
        self.params.subscription_items = Some(subscription_items);
        self
    }
    pub fn subscription_proration_behavior(
        mut self,
        subscription_proration_behavior: &str,
    ) -> Self {
        self
            .params
            .subscription_proration_behavior = Some(
            subscription_proration_behavior.to_owned(),
        );
        self
    }
    pub fn subscription_proration_date(
        mut self,
        subscription_proration_date: i64,
    ) -> Self {
        self.params.subscription_proration_date = Some(subscription_proration_date);
        self
    }
    pub fn subscription_resume_at(mut self, subscription_resume_at: &str) -> Self {
        self.params.subscription_resume_at = Some(subscription_resume_at.to_owned());
        self
    }
    pub fn subscription_start_date(mut self, subscription_start_date: i64) -> Self {
        self.params.subscription_start_date = Some(subscription_start_date);
        self
    }
    pub fn subscription_trial_end(
        mut self,
        subscription_trial_end: serde_json::Value,
    ) -> Self {
        self.params.subscription_trial_end = Some(subscription_trial_end);
        self
    }
    pub fn subscription_trial_from_plan(
        mut self,
        subscription_trial_from_plan: bool,
    ) -> Self {
        self.params.subscription_trial_from_plan = Some(subscription_trial_from_plan);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, GetInvoicesUpcomingRequest> {
    type Output = httpclient::InMemoryResult<Invoice>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = "/v1/invoices/upcoming";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}