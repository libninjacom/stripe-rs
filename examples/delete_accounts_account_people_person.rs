#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let account = "your account";
    let person = "your person";
    let response = client
        .delete_accounts_account_people_person(account, person)
        .await
        .unwrap();
    println!("{:#?}", response);
}