#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let session = "your session";
    let response = client
        .post_identity_verification_sessions_session_cancel(session)
        .await
        .unwrap();
    println!("{:#?}", response);
}