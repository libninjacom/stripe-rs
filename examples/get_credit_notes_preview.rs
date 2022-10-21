#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let invoice = "your invoice";
    let response = client
        .get_credit_notes_preview(invoice)
        .amount(1)
        .credit_amount(1)
        .expand(&["your expand"])
        .lines(vec![::serde_json::json!({})])
        .memo("your memo")
        .metadata(::serde_json::json!({}))
        .out_of_band_amount(1)
        .reason("your reason")
        .refund("your refund")
        .refund_amount(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
