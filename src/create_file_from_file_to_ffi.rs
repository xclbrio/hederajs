#![feature(async_await, futures_api, await_macro)]
use failure::{format_err, Error};
use futures::FutureExt;
use hedera::{Client, SecretKey, Status};
use std::{env, thread::sleep, time::Duration};
use std::str::FromStr;
use tokio::{await, run_async};
use std::io::prelude::*;
use std::fs::File;

async fn main_(input_operator: &str, input_address: &str, input_port: &str, input_operator_secret: &str, input_path_to_file: &str) -> Result<(), Error> {
    //pretty_env_logger::try_init()?;

    // Operator is the account that sends the transaction to the network
    // This account is charged for the transaction fee
    let operator = input_operator.parse()?;
    let client = Client::builder(format!("{}-{}-{}", input_address, ":", input_port))
        .node("0:0:3".parse()?)
        .operator(operator, || input_operator_secret)
        .build()?;

    let operator_secret : String = input_operator_secret?;
    let secret = SecretKey::from_str(&operator_secret)?;
    let public = secret.public();

    // load file from file system
    let mut my_file = File::open(&input_path_to_file)?;
    let mut file_contents = Vec::new();
    // read the whole file
    my_file.read_to_end(&mut file_contents)?;    

    // Create a file
    let id = await!(client
        .create_file()
        .expires_in(Duration::from_secs(2_592_000))
        .key(public)
        .contents(file_contents)
        .memo("[hedera-sdk-rust][example] create_file")
        .sign(&input_operator_secret?.parse()?) // sign as the owner of the file
        .execute_async())?;

    println!("creating file; transaction = {}", id);

    // If we got here we know we passed pre-check
    // Depending on your requirements that may be enough for some kinds of transactions
    sleep(Duration::from_secs(2));

    // Get the receipt and check the status to prove it was successful
    let receipt = await!(client.transaction(id).receipt().get_async())?;
    if receipt.status != Status::Success {
        Err(format_err!(
            "transaction has a non-successful status: {:?}",
            receipt.status
        ))?;
    }

    let file = receipt.file_id.unwrap();
    println!("file ID = {}", file);

    Ok(())
}

fn output_create_file_from_file(input_operator: &str, input_address: &str, input_port: &str, input_operator_secret: &str, input_path_to_file: &str) {
    run_async(main_(input_operator, input_address, input_port, input_operator_secret, input_path_to_file).map(|res| match res {
        Ok(_) => {}
        Err(err) => eprintln!("error: {}", err),
    }))
}
