use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let account = "your account";
    let response = client
        .post_accounts_account_login_links(account)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}