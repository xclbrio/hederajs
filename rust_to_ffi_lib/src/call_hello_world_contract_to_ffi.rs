#![feature(async_await, futures_api, await_macro)]

extern crate ethabi;

use std::fs::File;
use hex::{ToHex, FromHex};
use ethabi::param_type::{ParamType, Reader};
use ethabi::token::{Token, Tokenizer, StrictTokenizer, LenientTokenizer};
use ethabi::{encode, decode, Contract, Function, Event, Hash};
use failure::{format_err, Error};
use futures::FutureExt;
use hedera::{Client, Status, query};
use std::{env, thread::sleep, time::Duration};
use std::str::FromStr;
use tokio::{await, run_async};

async fn main_(input_operator: &str, input_address: &str, input_port: &str, input_operator_secret: &str, input_smart_contract_abi: &str, input_contract_id: &str, input_gas: &str) -> Result<(), Error> {
    //pretty_env_logger::try_init()?;

    // Operator is the account that sends the transaction to the network
    // This account is charged for the transaction fee
    let operator = input_operator?.parse()?;
    //let node_port : String = env::var("NODE_PORT")?;
    let client = Client::builder(format!("{}-{}-{}", input_address, ":", input_port))
        .node("0:0:3"?.parse()?)
        .operator(operator, || input_operator_secret)
        .build()?;

    let operator_secret : String = input_operator_secret?;

    // load ABI 
    let file = File::open(&input_smart_contract_abi)?;
	let contract = Contract::load(file);

    let contract = match contract {
        Ok(contract) => contract,
        Err(error) => {
            panic!("There was a problem loading the contract ABI {:?}", error)
        },
    };

    let function = contract.function("getInt");
    let function = match function {
        Ok(function) => function.clone(),
        Err(error) => {
            panic!("There was an error getting the getInt function from the contract ABI {:?}", error)
        },
    }; 

    // encode the function to a byte array
    // no parameters for this call
    let function_call = function.encode_input(&[]);
    let function_call = match function_call {
        Ok(function_call) => function_call,
        Err(error) => {
            panic!("There was an error encoding the function {:?}", error)
        },
    };

    // Call a contract function
    let id = await!(client
        .call_contract(input_contract_id?.parse()?)
        .gas(input_gas?.parse::<i64>()?)
        .amount(0)
        .function_parameters(function_call)
        .memo("[hedera-sdk-rust][example] call_contract")
        .generate_record(true)
        .execute_async())?;

    //     // .sign(&env::var("OPERATOR_SECRET")?.parse()?) // sign as the owner of the file

    println!("calling contract; transaction = {}", id);

    // If we got here we know we passed pre-check
    // Depending on your requirements that may be enough for some kinds of transactions
    sleep(Duration::from_secs(5));

    // get the record from the contract call and extract the result
    let record = await!(client.transaction(id).record().get_async())?;
    if record.receipt.status != Status::Success {
        Err(format_err!(
            "transaction has a non-successful status: {:?}",
            record.receipt.status
        ))?;
    }

    println!("Got record body {:?}", record.contract_function_result);
    println!("Got call result {:?}", record.contract_function_result.contract_id);

    let contract_function_result = record.contract_function_result; //.contract_call_result;


    // get the byte array containing results from the record
    // match record.body {
    //     query::ContractFunctionResult { _, contract_call_result, error_message, bloom, gas_used, log_info } => println!("Record is: {:?}", contract_call_result),
    //     _ => println!("Not what I expected"),
    // }

    // let function_result = function.decode_output(result);
    // let function_result = match function_result {
    //     Ok(function_result) => function_result,
    //     Err(error) => {
    //         panic!("There was an error decoding the function result {:?}", error)
    //     },
    // };

    // println!("result = {}", function_result.unwrap());

    Ok(())
}

fn output_call_hello_world_contract(input_operator: &str, input_address: &str, input_port: &str, input_operator_secret: &str, input_smart_contract_abi: &str, input_contract_id: &str, input_gas: &str) {
    run_async(main_(input_operator, input_address, input_port, input_operator_secret, input_smart_contract_abi, input_contract_id, input_gas).map(|res| match res {
        Ok(_) => {}
        Err(err) => eprintln!("error: {}", err),
    }))
}
