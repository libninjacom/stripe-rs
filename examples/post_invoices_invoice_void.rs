#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let invoice = "your invoice";
    let response = client.post_invoices_invoice_void(invoice).await.unwrap();
    println!("{:#?}", response);
}