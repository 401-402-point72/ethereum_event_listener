// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#![allow(clippy::result_large_err)]

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::{config::Region, meta::PKG_VERSION, Client, Error};
use clap::Parser;
use std::io;
use std::path::Path;
use std::process;

#[derive(Debug, Parser)]
struct Opt {
    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// The name of the bucket.
    #[structopt(short, long)]
    bucket: String,

    /// The name of the file to upload.
    #[structopt(short, long)]
    filename: String,

    /// The name of the object in the bucket.
    #[structopt(short, long)]
    key: String,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

// Upload a file to a bucket.
// snippet-start:[s3.rust.s3-helloworld]
async fn upload_object(
    client: &Client,
    bucket: &str,
    filename: &str,
    key: &str,
) -> Result<(), Error> {
    let resp = client.list_buckets().send().await?;

    for bucket in resp.buckets() {
        println!("bucket: {:?}", bucket.name().unwrap_or_default())
    }
    println!();

    let body = ByteStream::from_path(Path::new(filename)).await;

    match body {
        Ok(b) => {
            let resp = client
                .put_object()
                .bucket(bucket)
                .key(key)
                .body(b)
                .send()
                .await?;
            println!("Upload success. Version: {:?}", resp.version_id);
            let resp = client.get_object().bucket(bucket).key(key).send().await?;
            let data = resp.body.collect().await;
            // println!("data: {:?}", data.unwrap().into_bytes());
        }
        Err(e) => {
            println!("Got an error uploading object:");
            println!("{}", e);
            process::exit(1);
        }
    }
    Ok(())
}

/*
 * snippet-end:[s3.rust.s3-helloworld]
 * Lists your buckets and uploads a file to a bucket.
 *
 * # Arguments
 * `-b BUCKET` - The bucket to which the file is uploaded.
 * `-k KEY` - The name of the file to upload to the bucket.
 * `[-r REGION]` - The Region in which the client is created.
 *    If not supplied, uses the value of the **AWS_REGION** environment variable.
 *    If the environment variable is not set, defaults to **us-east-1**.
 * `[-v]` - Whether to display additional information.
 */
#[tokio::main]
pub async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    println!("Enter bucket name:");
    let mut bucket = String::new();
    io::stdin()
        .read_line(&mut bucket)
        .expect("Failed to read line");
    let bucket = bucket.trim();

    println!("Enter filename:");
    let mut filename = String::new();
    io::stdin()
        .read_line(&mut filename)
        .expect("Failed to read line");
    let filename = filename.trim();

    println!("Enter key:");
    let mut key = String::new();
    io::stdin()
        .read_line(&mut key)
        .expect("Failed to read line");
    let key = key.trim();

    let region_provider = RegionProviderChain::first_try(Region::new("us-east-1"));
    println!();

    println!("S3 client version: {}", PKG_VERSION);
    println!(
        "Region:            {}",
        region_provider.region().await.unwrap().as_ref()
    );
    println!("Bucket:            {}", &bucket);
    println!("Filename:          {}", &filename);
    println!("Key:               {}", &key);
    println!();

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    upload_object(&client, &bucket, &filename, &key).await
}