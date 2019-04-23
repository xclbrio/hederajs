#![feature(async_await, futures_api, await_macro)]
use failure::{format_err, Error};
use futures::FutureExt;
use hedera::{AccountId, Client, Status};
use std::{env, thread::sleep, time::Duration};
use tokio::{await, run_async};

async fn main_(input_operator: &str, input_address: &str, input_port: &str, input_operator_secret: &str, input_receiver_secret: &str, input_account_id: &str) -> Result<(), Error> {
    //pretty_env_logger::try_init()?;

    // Operator is the account that sends the transaction to the network
    // This account is charged for the transaction fee
    let operator = input_operator?.parse()?;
    let client = Client::builder(format!("{}-{}-{}", input_address, ":", input_port))
        .node("0:0:3".parse()?)
        .operator(operator, || input_operator_secret)
        .build()?;

    // Receiver is the account that receives the transferred crypto
    let receiver: AccountId = input_account_id.parse()?;

    // transfer 1 hbar from the operator account to the receiver account.
    let id = await!(client
        .transfer_crypto()
        .transfer(operator, -1_000_000)
        .transfer(receiver, 1_000_000)
        .memo("[hedera-sdk-rust][example] transfer_crypto")
        .sign(&input_operator_secret?.parse()?)
        .sign(&input_receiver_secret?.parse()?)
        .execute_async())?;

    println!("created transfer; transaction = {}", id);

    // If we got here we know we passed pre-check
    // Depending on your requirements that may be enough for some kinds of transactions
    sleep(Duration::from_secs(5));

    // Get the receipt and check the status to prove it was successful
    let receipt = await!(client.transaction(id).receipt().get_async())?;

    if receipt.status != Status::Success {
        Err(format_err!(
            "transaction has a non-successful status: {:?}",
            receipt.status
        ))?;
    }

    Ok(())
}

fn output_transfer_crypto(input_operator: &str, input_address: &str, input_port: &str, input_operator_secret: &str, input_receiver_secret: &str, input_account_id: &str) {
    run_async(main_(input_operator, input_address, input_port, input_operator_secret, input_receiver_secret, input_account_id).map(|res| match res {
        Ok(_) => {}
        Err(err) => eprintln!("error: {}", err),
    }))
}
