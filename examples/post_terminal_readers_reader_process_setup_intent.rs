use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let reader = "your reader";
    let response = client
        .post_terminal_readers_reader_process_setup_intent(reader)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
