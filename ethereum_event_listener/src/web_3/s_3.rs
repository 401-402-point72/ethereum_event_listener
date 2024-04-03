// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#![allow(clippy::result_large_err)]

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::{config::Region, Client};
use serde_json::Value;
use std::env;

pub async fn init_connection() -> (String, Client) {
    // Pull in environment variables
    dotenv::dotenv().ok();

    let bucket_name = &env::var("BUCKET_NAME").unwrap();
    let region_provider = RegionProviderChain::first_try(Region::new("us-east-1"));

    println!("Bucket Name: {}", bucket_name);
    println!("Region: {}", region_provider.region().await.unwrap());

    // region_provider somehow gets borrowed so gotta print first ... weird rust bs
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    (bucket_name.to_string(), client)
}

// Upload a file to a bucket.
#[tokio::main]
pub async fn upload_object(client: &Client, bucket: &str, block: &Value) -> () {
    // println!("{:#}", block);
    if let Some(block_number) = block.get("blockNumber") {
        if let Some(block_number_str) = block_number.as_str() {
            println!("blockNumber: {}", block_number_str);
        }
    }

    // Convert json object to rust native byte stream and then aws byte stream
    let rust_bytestream = serde_json::to_vec(&block).unwrap();
    let aws_bytestream = ByteStream::from(rust_bytestream);

    // Grab block number as indexable key
    let key = match block["blockNumber"].as_str() {
        Some(value) => value,
        None => {
            println!("Block number not found or is not a string");
            return;
        }
    };

    // Store object in bucket ... YAY!
    let _response = client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(aws_bytestream)
        .send()
        .await;

    ()
}
