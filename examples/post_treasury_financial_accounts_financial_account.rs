use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let financial_account = "your financial account";
    let response = client
        .post_treasury_financial_accounts_financial_account(financial_account)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
