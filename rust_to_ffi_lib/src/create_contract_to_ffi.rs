#![feature(async_await, futures_api, await_macro)]
use failure::{format_err, Error};
use futures::FutureExt;
use hedera::{Client, SecretKey, Status};
use std::{env, thread::sleep, time::Duration};
use std::str::FromStr;
use tokio::{await, run_async};

async fn main_(input_operator: &str, input_address: &str, input_port: &str, input_operator_secret: &str, input_file_id: &str, input_gas: &str) -> Result<(), Error> {
    //pretty_env_logger::try_init()?;

    // Operator is the account that sends the transaction to the network
    // This account is charged for the transaction fee
    let operator = input_operator?.parse()?;
    //let node_port : String = env::var("NODE_PORT")?;
    let client = Client::builder(format!("{}-{}-{}", input_address, ":", input_port))
        .node("0:0:3".parse()?)
        .operator(operator, || input_operator_secret)
        .build()?;

    let operator_secret : String = input_operator_secret?;
    let file_id = input_file_id?.parse()?;
    let gas = input_gas?.parse::<i64>()?;

    // Create a contract from an existing Hedera file
    let id = await!(client
        .create_contract()
        .file(file_id)
        .gas(gas)
        .auto_renew_period(Duration::from_secs(2_592_000))
        .memo("[hedera-sdk-rust][example] create_contract")
        .execute_async())?;

    println!("creating contract; transaction = {}", id);

        //.constructor_parameters(params: Vec<u8>)
        // .initial_balance(balance: i64)
        // .sign(&env::var("OPERATOR_SECRET")?.parse()?) // sign as the owner of the file

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

    let contract = receipt.contract_id.unwrap();
    
    println!("contract ID = {}", contract);
    println!("Run these (OS Depending) to run further contract examples");
    println!("export CONTRACT_ID={}", contract);
    println!("set CONTRACT_ID={}", contract);

    Ok(())
}

fn output_create_contract(input_operator: &str, input_address: &str, input_port: &str, input_operator_secret: &str, input_file_id: &str, input_gas: &str) {
    run_async(main_(input_operator, input_address, input_port, input_operator_secret, input_file_id, input_gas).map(|res| match res {
        Ok(_) => {}
        Err(err) => eprintln!("error: {}", err),
    }))
}
