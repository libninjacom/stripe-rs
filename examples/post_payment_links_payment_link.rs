#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let payment_link = "your payment link";
    let response = client
        .post_payment_links_payment_link(payment_link)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
