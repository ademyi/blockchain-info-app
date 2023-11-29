use dotenv;
use reqwest;
use tokio;
use serde_json::Result;
use crate::blockchain_status::BlockchainStatus;
use crate::blockchain_address::BlockchainAddress;
use crate::blockchain_transactions::BlockchainTransaction;

const HOST_ROOT: &str = "https://btcbook.nownodes.io/api/";

#[tokio::main]
pub async fn send_request(url: &str) -> String {

    let client = reqwest::Client::new();
    client
        .get(url)
        .header("api-key", dotenv::var("API_KEY").expect("Could not find API_KEY"))
        .send()
        .await
        .expect("Failed to get response")
        .text()
        .await
        .expect("Failed to convert payload")

}

pub fn block_chain_status_request() -> BlockchainStatus {
    let response = send_request(&HOST_ROOT);
   // println!("{}", response);
    serde_json::from_str(&response).expect("Filed to parse JSON")
}

pub fn block_chain_address_request(address : &str) -> BlockchainAddress {
    let response = send_request(&[HOST_ROOT, "v2/address/", &address].join(""));
    // println!("{}", response);
    serde_json::from_str(&response).expect("Filed to parse JSON")
}

pub fn block_transaction_request(transaction : &str) -> BlockchainTransaction {
    let response = send_request(&[HOST_ROOT, "v2/tx/", &transaction].join(""));
    // println!("{}", response);
    serde_json::from_str(&response).expect("Filed to parse JSON")
}
