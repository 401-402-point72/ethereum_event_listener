// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#![allow(clippy::result_large_err)]

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::{config::Region, meta::PKG_VERSION, Client, Error};
use web3::types::{BlockId, BlockNumber, H256, U256, U64};
use std::env;
use std::path::Path;
use std::process;
use serde_json::{json,Value};

pub async fn init_connection() -> (String, Client) {
    dotenv::dotenv().ok();

    let bucket_name = &env::var("BUCKET_NAME").unwrap();
    let region_provider = RegionProviderChain::first_try(Region::new("us-east-1"));

    println!("Bucket Name: {}", bucket_name);
    println!(
        "Region: {}",
        region_provider.region().await.unwrap()
    );

    // region_provider somehow gets borrowed so gotta print first ... weird rust bs
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    (bucket_name.to_string(), client)
}

// Upload a file to a bucket.
// snippet-start:[s3.rust.s3-helloworld]
#[tokio::main]
pub async fn upload_object(
    client: &Client,
    bucket: &str,
    block: &Value
) -> Result<(), Error> {
    let resp = client.list_buckets().send().await;

    // println!("In thread");

    println!("{:#}", block);
   
    // let body = ByteStream::from_path(Path::new(filename)).await;

    // match body {
    //     Ok(b) => {
    //         let resp = client
    //             .put_object()
    //             .bucket(bucket)
    //             .key(key)
    //             .body(b)
    //             .send()
    //             .await?;
    //         println!("Upload success. Version: {:?}", resp.version_id);
    //         let resp = client.get_object().bucket(bucket).key(key).send().await?;
    //         let data = resp.body.collect().await;
    //         // println!("data: {:?}", data.unwrap().into_bytes());
    //     }
    //     Err(e) => {
    //         println!("Got an error uploading object:");
    //         println!("{}", e);
    //         process::exit(1);
    //     }
    // }
    Ok(())
}