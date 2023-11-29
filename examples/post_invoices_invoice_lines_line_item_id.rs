#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let invoice = "your invoice";
    let line_item_id = "your line item id";
    let response = client
        .post_invoices_invoice_lines_line_item_id(invoice, line_item_id)
        .await
        .unwrap();
    println!("{:#?}", response);
}