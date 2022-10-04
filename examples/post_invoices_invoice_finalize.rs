use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let invoice = "your invoice";
    let response = client.post_invoices_invoice_finalize(invoice).send().await.unwrap();
    println!("{:#?}", response);
}
