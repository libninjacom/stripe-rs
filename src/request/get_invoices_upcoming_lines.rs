use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetInvoicesUpcomingLinesRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub automatic_tax: Option<AutomaticTaxParam>,
    pub coupon: Option<String>,
    pub currency: Option<String>,
    pub customer: Option<String>,
    pub customer_details: Option<CustomerDetailsParam>,
    pub discounts: Option<serde_json::Value>,
    pub ending_before: Option<String>,
    pub expand: Option<Vec<String>>,
    pub invoice_items: Option<Vec<InvoiceItemPreviewParams>>,
    pub limit: Option<i64>,
    pub schedule: Option<String>,
    pub starting_after: Option<String>,
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
impl<'a> GetInvoicesUpcomingLinesRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<InvoiceLinesList> {
        let mut r = self.http_client.client.get("/v1/invoices/upcoming/lines");
        if let Some(ref unwrapped) = self.automatic_tax {
            r = r.query("automatic_tax", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.coupon {
            r = r.query("coupon", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.currency {
            r = r.query("currency", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.customer {
            r = r.query("customer", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.customer_details {
            r = r.query("customer_details", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.discounts {
            r = r.query("discounts", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ending_before {
            r = r.query("ending_before", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.invoice_items {
            for item in unwrapped {
                r = r.query("invoice_items[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.schedule {
            r = r.query("schedule", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.starting_after {
            r = r.query("starting_after", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription {
            r = r.query("subscription", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_billing_cycle_anchor {
            r = r.query("subscription_billing_cycle_anchor", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_cancel_at {
            r = r.query("subscription_cancel_at", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_cancel_at_period_end {
            r = r.query("subscription_cancel_at_period_end", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_cancel_now {
            r = r.query("subscription_cancel_now", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_default_tax_rates {
            r = r.query("subscription_default_tax_rates", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_items {
            for item in unwrapped {
                r = r.query("subscription_items[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.subscription_proration_behavior {
            r = r.query("subscription_proration_behavior", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_proration_date {
            r = r.query("subscription_proration_date", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_resume_at {
            r = r.query("subscription_resume_at", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_start_date {
            r = r.query("subscription_start_date", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_trial_end {
            r = r.query("subscription_trial_end", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.subscription_trial_from_plan {
            r = r.query("subscription_trial_from_plan", &unwrapped.to_string());
        }
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
    pub fn automatic_tax(mut self, automatic_tax: AutomaticTaxParam) -> Self {
        self.automatic_tax = Some(automatic_tax);
        self
    }
    pub fn coupon(mut self, coupon: &str) -> Self {
        self.coupon = Some(coupon.to_owned());
        self
    }
    pub fn currency(mut self, currency: &str) -> Self {
        self.currency = Some(currency.to_owned());
        self
    }
    pub fn customer(mut self, customer: &str) -> Self {
        self.customer = Some(customer.to_owned());
        self
    }
    pub fn customer_details(mut self, customer_details: CustomerDetailsParam) -> Self {
        self.customer_details = Some(customer_details);
        self
    }
    pub fn discounts(mut self, discounts: serde_json::Value) -> Self {
        self.discounts = Some(discounts);
        self
    }
    pub fn ending_before(mut self, ending_before: &str) -> Self {
        self.ending_before = Some(ending_before.to_owned());
        self
    }
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn invoice_items(
        mut self,
        invoice_items: Vec<InvoiceItemPreviewParams>,
    ) -> Self {
        self.invoice_items = Some(invoice_items);
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn schedule(mut self, schedule: &str) -> Self {
        self.schedule = Some(schedule.to_owned());
        self
    }
    pub fn starting_after(mut self, starting_after: &str) -> Self {
        self.starting_after = Some(starting_after.to_owned());
        self
    }
    pub fn subscription(mut self, subscription: &str) -> Self {
        self.subscription = Some(subscription.to_owned());
        self
    }
    pub fn subscription_billing_cycle_anchor(
        mut self,
        subscription_billing_cycle_anchor: serde_json::Value,
    ) -> Self {
        self.subscription_billing_cycle_anchor = Some(subscription_billing_cycle_anchor);
        self
    }
    pub fn subscription_cancel_at(
        mut self,
        subscription_cancel_at: serde_json::Value,
    ) -> Self {
        self.subscription_cancel_at = Some(subscription_cancel_at);
        self
    }
    pub fn subscription_cancel_at_period_end(
        mut self,
        subscription_cancel_at_period_end: bool,
    ) -> Self {
        self.subscription_cancel_at_period_end = Some(subscription_cancel_at_period_end);
        self
    }
    pub fn subscription_cancel_now(mut self, subscription_cancel_now: bool) -> Self {
        self.subscription_cancel_now = Some(subscription_cancel_now);
        self
    }
    pub fn subscription_default_tax_rates(
        mut self,
        subscription_default_tax_rates: serde_json::Value,
    ) -> Self {
        self.subscription_default_tax_rates = Some(subscription_default_tax_rates);
        self
    }
    pub fn subscription_items(
        mut self,
        subscription_items: Vec<SubscriptionItemUpdateParams>,
    ) -> Self {
        self.subscription_items = Some(subscription_items);
        self
    }
    pub fn subscription_proration_behavior(
        mut self,
        subscription_proration_behavior: &str,
    ) -> Self {
        self
            .subscription_proration_behavior = Some(
            subscription_proration_behavior.to_owned(),
        );
        self
    }
    pub fn subscription_proration_date(
        mut self,
        subscription_proration_date: i64,
    ) -> Self {
        self.subscription_proration_date = Some(subscription_proration_date);
        self
    }
    pub fn subscription_resume_at(mut self, subscription_resume_at: &str) -> Self {
        self.subscription_resume_at = Some(subscription_resume_at.to_owned());
        self
    }
    pub fn subscription_start_date(mut self, subscription_start_date: i64) -> Self {
        self.subscription_start_date = Some(subscription_start_date);
        self
    }
    pub fn subscription_trial_end(
        mut self,
        subscription_trial_end: serde_json::Value,
    ) -> Self {
        self.subscription_trial_end = Some(subscription_trial_end);
        self
    }
    pub fn subscription_trial_from_plan(
        mut self,
        subscription_trial_from_plan: bool,
    ) -> Self {
        self.subscription_trial_from_plan = Some(subscription_trial_from_plan);
        self
    }
}
impl<'a> ::std::future::IntoFuture for GetInvoicesUpcomingLinesRequest<'a> {
    type Output = httpclient::InMemoryResult<InvoiceLinesList>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}