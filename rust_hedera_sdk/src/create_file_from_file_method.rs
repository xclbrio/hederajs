#![feature(async_await, futures_api, await_macro)]
use failure::{format_err, Error};
use futures::FutureExt;
use crate::{Client, SecretKey, Status};
use std::{env, thread::sleep, time::Duration};
use std::str::FromStr;
use tokio::{await, run_async};
use std::io::prelude::*;
use std::fs::File;

pub fn create_file_from_file_func<'a>(input_operator: &str, input_node_port: &str, input_node_account: &str, input_private_key: &'static str, input_file_path: &'static str) -> &'a str  {
    //pretty_env_logger::try_init()?;

    // Operator is the account that sends the transaction to the network
    // This account is charged for the transaction fee
    let operator = input_operator.parse().unwrap();
    let client = Client::builder(input_node_port)
        .node(input_node_account.parse().unwrap())
        .operator(operator, move || input_private_key)
        .build().unwrap();

    let operator_secret : String = input_private_key.to_string();
    let secret = SecretKey::from_str(&operator_secret).unwrap();
    let public = secret.public();

    // load file from file system
    let mut my_file = File::open(input_file_path).unwrap();
    let mut file_contents = Vec::new();
    // read the whole file
    my_file.read_to_end(&mut file_contents).unwrap();

    // Create a file
    let id = client
        .create_file()
        .expires_in(Duration::from_secs(2_592_000))
        .key(public)
        .contents(file_contents)
        .memo("[hedera-sdk-rust][example] create_file")
        .sign(&input_private_key.to_string().parse().unwrap()).execute().unwrap();

    println!("creating file; transaction = {}", id);

    // If we got here we know we passed pre-check
    // Depending on your requirements that may be enough for some kinds of transactions
    sleep(Duration::from_secs(4));

    // Get the receipt and check the status to prove it was successful
    let receipt = client.transaction(id).receipt().get().unwrap();
//    if receipt.status != Status::Success {
//        Err(format_err!(
//            "transaction has a non-successful status: {:?}",
//            receipt.status.unwrap()
//        )).unwrap();
//    }

    let file = receipt.file_id.unwrap();
    println!("file ID = {}", file);

    "Result returned from create_file_from_file_method"
}
