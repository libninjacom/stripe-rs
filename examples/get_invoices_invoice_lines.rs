use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let invoice = "your invoice";
    let response = client
        .get_invoices_invoice_lines(invoice)
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .starting_after("your starting after")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
