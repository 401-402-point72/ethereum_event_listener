mod s_3;

use chrono::{DateTime, Local, TimeZone};
use std::env;
use std::str::FromStr;
use std::thread;
use std::time::Duration;
use web3::types::{BlockId, BlockNumber, H160, H256, U256, U64};
use serde_json::{json,Value};
use std::sync::Arc;

fn wei_to_eth(wei_val: U256) -> f64 {
    let res = wei_val.as_u128() as f64;
    res / 1_000_000_000_000_000_000.0
}

fn convert_date(timestamp_str: &str) -> DateTime<Local> {
    if let Ok(timestamp) = timestamp_str.parse::<i64>() {
        return Local.timestamp(timestamp, 0);
    } else {
        return Local.timestamp(0, 0);
    }
}

fn format_as_json(block: &web3::types::Block<H256>) -> Value {
    // println!("In format");
    
    let block_as_json = json!({
        "blockHash": block.hash,
        "blockNumber": block.number,
        "numberOfTransactions": block.transactions.len(),
        "blockGasUsed": block.gas_used,
        "difficulty": block.total_difficulty.unwrap(),
        "timestamp": block.timestamp.to_string(),
        "authorAddress": block.author,
    });

    (block_as_json)
}

#[tokio::main]
pub async fn read_block_data() -> web3::Result<()> {
    //process setup, connect to S3 and get environment secrets
    let (bucket, client) = s_3::init_connection().await;
    dotenv::dotenv().ok();

    // Build the connection to the network
    let websocket = web3::transports::WebSocket::new(&env::var("INFURA_SEPOLIA").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    // Get accounts from the connected node
    let mut accounts = web3s.eth().accounts().await?;
    accounts.push(H160::from_str(&env::var("ACCOUNT_ADDRESS").unwrap()).unwrap());
    println!("Accounts: {:?}", accounts);

    // Print those accounts' balances converted to Eth
    for account in accounts {
        let balance = web3s.eth().balance(account, None).await?;
        println!("Eth balance of {:?} {}", account, wei_to_eth(balance));
    }

    // Used for caching latest block number
    let mut previous_block_number: U64 = U64([u64::min_value(); 1]);

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

            let bucket_clone = Arc::clone(&bucket_arc);
            let client_clone = Arc::clone(&client_arc);

            let _ = thread::spawn(move || {
                // println!("Spawned a thread to store the data");
                let block_json = format_as_json(&latest_block);
                s_3::upload_object(&client_clone, &bucket_clone, &block_json);
            });
        }
        previous_block_number = block_number;

        // limits the number of requests we make
        thread::sleep(Duration::from_secs(1));
    }
    Ok(())
}
