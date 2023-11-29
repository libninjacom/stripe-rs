#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let scope = ScopeParam {
        type_: "your type".to_owned(),
        user: Some("your user".to_owned()),
    };
    let response = client
        .get_apps_secrets(scope)
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .starting_after("your starting after")
        .await
        .unwrap();
    println!("{:#?}", response);
}