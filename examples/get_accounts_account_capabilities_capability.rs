use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let account = "your account";
    let capability = "your capability";
    let response = client
        .get_accounts_account_capabilities_capability(account, capability)
        .expand(&["your expand"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
