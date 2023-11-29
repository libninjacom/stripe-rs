#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let account = "your account";
    let person = "your person";
    let response = client
        .post_accounts_account_persons_person(account, person)
        .await
        .unwrap();
    println!("{:#?}", response);
}