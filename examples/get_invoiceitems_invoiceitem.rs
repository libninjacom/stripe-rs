#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let invoiceitem = "your invoiceitem";
    let response = client
        .get_invoiceitems_invoiceitem(invoiceitem)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}