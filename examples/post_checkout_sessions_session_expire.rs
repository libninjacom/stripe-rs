#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let session = "your session";
    let response = client.post_checkout_sessions_session_expire(session).await.unwrap();
    println!("{:#?}", response);
}