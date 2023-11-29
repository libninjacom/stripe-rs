#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let review = "your review";
    let response = client
        .get_reviews_review(review)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}