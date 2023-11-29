#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let account = "your account";
    let response = client
        .get_accounts_account_people(account)
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .relationship(AllPeopleRelationshipSpecs {
            director: Some(true),
            executive: Some(true),
            legal_guardian: Some(true),
            owner: Some(true),
            representative: Some(true),
        })
        .starting_after("your starting after")
        .await
        .unwrap();
    println!("{:#?}", response);
}