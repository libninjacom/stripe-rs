use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let account = "your account";
    let id = "your id";
    let response = client
        .post_accounts_account_bank_accounts_id(account, id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
