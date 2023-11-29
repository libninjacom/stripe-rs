#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_invoices_upcoming_lines()
        .automatic_tax(AutomaticTaxParam { enabled: true })
        .coupon("your coupon")
        .currency("your currency")
        .customer("your customer")
        .customer_details(CustomerDetailsParam {
            address: Some(serde_json::json!({})),
            shipping: Some(serde_json::json!({})),
            tax: Some(TaxParam {
                ip_address: Some(serde_json::json!({})),
            }),
            tax_exempt: Some("your tax exempt".to_owned()),
            tax_ids: Some(
                vec![
                    DataParams { type_ : "your type".to_owned(), value : "your value"
                    .to_owned() }
                ],
            ),
        })
        .discounts(serde_json::json!({}))
        .ending_before("your ending before")
        .expand(&["your expand"])
        .invoice_items(
            vec![
                InvoiceItemPreviewParams { amount : Some(1), currency :
                Some("your currency".to_owned()), description : Some("your description"
                .to_owned()), discountable : Some(true), discounts :
                Some(serde_json::json!({})), invoiceitem : Some("your invoiceitem"
                .to_owned()), metadata : Some(serde_json::json!({})), period :
                Some(Period { end : 1, start : 1 }), price : Some("your price"
                .to_owned()), price_data : Some(OneTimePriceData { currency :
                "your currency".to_owned(), product : "your product".to_owned(),
                tax_behavior : Some("your tax behavior".to_owned()), unit_amount :
                Some(1), unit_amount_decimal : Some(rust_decimal_macros::dec!(100.01))
                }), quantity : Some(1), tax_behavior : Some("your tax behavior"
                .to_owned()), tax_code : Some(serde_json::json!({})), tax_rates :
                Some(serde_json::json!({})), unit_amount : Some(1), unit_amount_decimal :
                Some(rust_decimal_macros::dec!(100.01)) }
            ],
        )
        .limit(1)
        .schedule("your schedule")
        .starting_after("your starting after")
        .subscription("your subscription")
        .subscription_billing_cycle_anchor(serde_json::json!({}))
        .subscription_cancel_at(serde_json::json!({}))
        .subscription_cancel_at_period_end(true)
        .subscription_cancel_now(true)
        .subscription_default_tax_rates(serde_json::json!({}))
        .subscription_items(
            vec![
                SubscriptionItemUpdateParams { billing_thresholds :
                Some(serde_json::json!({})), clear_usage : Some(true), deleted :
                Some(true), id : Some("your id".to_owned()), metadata :
                Some(serde_json::json!({})), price : Some("your price".to_owned()),
                price_data : Some(RecurringPriceData { currency : "your currency"
                .to_owned(), product : "your product".to_owned(), recurring :
                RecurringAdhoc { interval : "your interval".to_owned(), interval_count :
                Some(1) }, tax_behavior : Some("your tax behavior".to_owned()),
                unit_amount : Some(1), unit_amount_decimal :
                Some(rust_decimal_macros::dec!(100.01)) }), quantity : Some(1), tax_rates
                : Some(serde_json::json!({})) }
            ],
        )
        .subscription_proration_behavior("your subscription proration behavior")
        .subscription_proration_date(1)
        .subscription_resume_at("your subscription resume at")
        .subscription_start_date(1)
        .subscription_trial_end(serde_json::json!({}))
        .subscription_trial_from_plan(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}