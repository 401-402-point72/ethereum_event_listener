mod s_3;

use chrono::{DateTime, Local, TimeZone};
use std::env;
use std::thread;
use std::time::Duration;
use web3::types::{BlockId, BlockNumber, H256, U256, U64};
use serde_json::{json,Value};
use std::sync::Arc;

fn convert_date(timestamp_str: &str) -> DateTime<Local> {
    if let Ok(timestamp) = timestamp_str.parse::<i64>() {
        return Local.timestamp(timestamp, 0);
    } else {
        return Local.timestamp(0, 0);
    }
}

fn format_as_json(block: &web3::types::Block<H256>) -> Value {    
    let block_as_json = json!({
        "blockHash": block.hash,
        "blockNumber": block.number,
        "numberOfTransactions": block.transactions.len(),
        "blockGasUsed": block.gas_used,
        "difficulty": block.total_difficulty.unwrap(),
        "timestamp": block.timestamp.to_string(),
        "authorAddress": block.author,
    });

    block_as_json
}

#[tokio::main]
pub async fn read_block_data() -> web3::Result<()> {
    //process setup, connect to S3 and get environment secrets
    let (bucket, client) = s_3::init_connection().await;
    dotenv::dotenv().ok();

    // Build the connection to the network
    let websocket = web3::transports::WebSocket::new(&env::var("INFURA_SEPOLIA").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    // Used for caching latest block number
    let mut previous_block_number: U64 = U64([u64::min_value(); 1]);

    // Use Arc clones to stop borrowing issues
    let bucket_arc = Arc::new(bucket.clone());
    let client_arc = Arc::new(client.clone());

    loop {
        // Get the latest block
        let latest_block = web3s
            .eth()
            .block(BlockId::Number(BlockNumber::Latest))
            .await
            .unwrap()
            .unwrap();
        let block_number = latest_block.number.unwrap();

        // Do not print block if that one was already printed
        if block_number > previous_block_number {
            println!(
                "block number {}, number of transactions: {}, difficulty {} @ {}",
                latest_block.number.unwrap(),
                &latest_block.transactions.len(),
                &latest_block.total_difficulty.unwrap(),
                convert_date(&latest_block.timestamp.to_string())
            );

            // Clone again, don't ask me Rust is weird
            let bucket_clone = Arc::clone(&bucket_arc);
            let client_clone = Arc::clone(&client_arc);

            // Spawn thread to process data so we do not miss new blocks coming in
            let _ = thread::spawn(move || {
                let block_json = format_as_json(&latest_block);
                let _ = s_3::upload_object(&client_clone, &bucket_clone, &block_json);
            });
        }
        previous_block_number = block_number;

        // limits the number of requests we make
        thread::sleep(Duration::from_secs(1));
    }
}
