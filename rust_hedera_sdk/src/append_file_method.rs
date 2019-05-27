#![feature(async_await, futures_api, await_macro)]
use failure::{format_err, Error};
use futures::FutureExt;
use crate::{Client, Status, string_to_static_str};
use std::{str, env, thread::sleep, time::Duration};
use tokio::{await, run_async};

pub fn append_file_func<'a>(input_operator: &str, input_node_port: &str, input_node_account: &str, input_private_key: &'static str, input_file_id: &'static str, input_append_text: &'static str) -> &'a str  {

    // Operator is the account that sends the transaction to the network
    // This account is charged for the transaction fee
    let operator = input_operator.parse().unwrap();
    let client = Client::builder(input_node_port)
        .node(input_node_account.parse().unwrap())
        .operator(operator, move || input_private_key)
        .build().unwrap();

    // append to a file
    let file = input_file_id.parse().unwrap();

    let file_extra_string = input_append_text.to_string();
    let file_extra_bytes = file_extra_string.into_bytes();

    let id = client
        .append_file(file, file_extra_bytes)
        .sign(&input_private_key.to_string().parse().unwrap())
        .execute().unwrap();

    //println!("appending to file; transaction = {}", id);

    // If we got here we know we passed pre-check
    // Depending on your requirements that may be enough for some kinds of transactions
    sleep(Duration::from_secs(2));

    // Get the receipt and check the status to prove it was successful
    let receipt = client.transaction(id).receipt().get().unwrap();

    let file_contents = client.file(file).contents().get().unwrap();
    let converted_contents = str::from_utf8(&file_contents).unwrap().to_string();
    //println!(" = {:?}", converted_contents);
    //.transaction(id).receipt().get().unwrap();
//    if receipt.status != Status::Success {
//        Err(format_err!(
//            "transaction has a non-successful status: {:?}",
//            receipt.status
//        ))?;
//    }


    string_to_static_str(converted_contents)
}
