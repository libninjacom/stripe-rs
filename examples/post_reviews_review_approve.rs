#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let review = "your review";
    let response = client.post_reviews_review_approve(review).await.unwrap();
    println!("{:#?}", response);
}