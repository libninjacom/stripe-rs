#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let credit_reversal = "your credit reversal";
    let response = client
        .get_treasury_credit_reversals_credit_reversal(credit_reversal)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}