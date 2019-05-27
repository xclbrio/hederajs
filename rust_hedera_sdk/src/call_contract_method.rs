#![feature(async_await, futures_api, await_macro)]

extern crate ethabi;

use std::fs::File;
use hex::{ToHex, FromHex};
use ethabi::param_type::{ParamType, Reader};
use ethabi::token::{Token, Tokenizer, StrictTokenizer, LenientTokenizer};
use ethabi::{encode, decode, Contract, Function, Event, Hash};
use failure::{format_err, Error};
use futures::FutureExt;
use crate::{Client, Status, query, string_to_static_str};
use std::{env, thread::sleep, time::Duration};
use std::str::FromStr;
use tokio::{await, run_async};
use core::borrow::Borrow;

pub fn call_contract_func<'a>(input_operator: &str, input_node_port: &str, input_node_account: &str, input_private_key: &'static str, input_contract_id: &'static str, input_gas: &'static str, input_abi_path: &'static str, input_function: &'static str, input_amount: &'static str, input_arguments: &'static str) -> &'a str  {
    //println!("{:?}", input_arguments);
    // Operator is the account that sends the transaction to the network
    // This account is charged for the transaction fee
    let operator = input_operator.parse().unwrap();;

    let node_port : String = input_node_port.to_string();;
    let client = Client::builder(input_node_port)
        .node(input_node_account.parse().unwrap())
        .operator(operator, move || input_private_key)
        .build().unwrap();

    let operator_secret : String = input_private_key.to_string();;

    // load ABI 
    let file = File::open(input_abi_path).unwrap();
	let contract = Contract::load(file);
    let contract = match contract {
        Ok(contract) => contract,
        Err(error) => {
            panic!("There was a problem loading the contract ABI {:?}", error)
        },
    };

    let function = contract.function(input_function);
    let function = match function {
        Ok(function) => function.clone(),
        Err(error) => {
            panic!("There was an error getting the getInt function from the contract ABI {:?}", error)
        },
    }; 

    // encode the function to a byte array
    let mut args_vector:Vec<Token> = [].to_vec();
    if input_arguments != "" {
        let args_parsed = input_arguments
            .split(",").take(10)
            .collect::<Vec<&str>>();
        let temp = args_parsed
            .iter()
            .map(|x| Token::Uint(x.parse::<u32>().unwrap().into()))
            .collect::<Vec<Token>>();
        args_vector = temp;
        //println!("{:?}", args_parsed);
        //println!("{:?}", args_vector);
    }

    let function_call = function.encode_input(args_vector.as_slice());
    let function_call = match function_call {
        Ok(function_call) => function_call,
        Err(error) => {
            panic!("There was an error encoding the function {:?}", error)
        },
    };

    // Call a contract function
    let amount = input_amount.parse::<i64>().unwrap();
    let gas = input_gas.parse::<i64>().unwrap();
    let id = client
        .call_contract(input_contract_id.parse().unwrap())
        .gas(gas)
        .amount(amount)
        .function_parameters(function_call)
        .memo("[hedera-sdk-rust][example] call_contract")
        .generate_record(true)
        .execute().unwrap();



    println!("calling contract; transaction = {}", id);

    // If we got here we know we passed pre-check
    // Depending on your requirements that may be enough for some kinds of transactions
    sleep(Duration::from_secs(3));

    // get the record from the contract call and extract the result
    let record = client.transaction(id).record().get().unwrap();

    //возвращает корректно только числа
    //для нормальной работы нужен свич кейс
    let y = function.decode_output(&record.contract_function_result.unwrap().contract_call_result).unwrap()[0].clone().to_uint().unwrap().to_string();

    string_to_static_str(y)
}
