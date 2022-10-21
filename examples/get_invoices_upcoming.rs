#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_invoices_upcoming()
        .automatic_tax(::serde_json::json!({}))
        .coupon("your coupon")
        .currency("your currency")
        .customer("your customer")
        .customer_details(::serde_json::json!({}))
        .discounts(::serde_json::json!({}))
        .expand(&["your expand"])
        .invoice_items(vec![::serde_json::json!({})])
        .schedule("your schedule")
        .subscription("your subscription")
        .subscription_billing_cycle_anchor(::serde_json::json!({}))
        .subscription_cancel_at(::serde_json::json!({}))
        .subscription_cancel_at_period_end(true)
        .subscription_cancel_now(true)
        .subscription_default_tax_rates(::serde_json::json!({}))
        .subscription_items(vec![::serde_json::json!({})])
        .subscription_proration_behavior("your subscription proration behavior")
        .subscription_proration_date(1)
        .subscription_start_date(1)
        .subscription_trial_end(::serde_json::json!({}))
        .subscription_trial_from_plan(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
