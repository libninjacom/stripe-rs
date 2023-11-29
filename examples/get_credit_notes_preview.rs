#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let invoice = "your invoice";
    let response = client
        .get_credit_notes_preview(invoice)
        .amount(1)
        .credit_amount(1)
        .effective_at(1)
        .expand(&["your expand"])
        .lines(
            vec![
                CreditNoteLineItemParams { amount : Some(1), description :
                Some("your description".to_owned()), invoice_line_item :
                Some("your invoice line item".to_owned()), quantity : Some(1),
                tax_amounts : Some(serde_json::json!({})), tax_rates :
                Some(serde_json::json!({})), type_ : "your type".to_owned(), unit_amount
                : Some(1), unit_amount_decimal : Some(rust_decimal_macros::dec!(100.01))
                }
            ],
        )
        .memo("your memo")
        .metadata(serde_json::json!({}))
        .out_of_band_amount(1)
        .reason("your reason")
        .refund("your refund")
        .refund_amount(1)
        .shipping_cost(CreditNoteShippingCost {
            shipping_rate: Some("your shipping rate".to_owned()),
        })
        .await
        .unwrap();
    println!("{:#?}", response);
}