#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let reader = "your reader";
    let response = client
        .post_terminal_readers_reader_refund_payment(reader)
        .await
        .unwrap();
    println!("{:#?}", response);
}