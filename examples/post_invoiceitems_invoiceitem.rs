#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let invoiceitem = "your invoiceitem";
    let response = client.post_invoiceitems_invoiceitem(invoiceitem).await.unwrap();
    println!("{:#?}", response);
}