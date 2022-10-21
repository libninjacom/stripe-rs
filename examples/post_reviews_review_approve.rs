#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let review = "your review";
    let response = client.post_reviews_review_approve(review).send().await.unwrap();
    println!("{:#?}", response);
}
