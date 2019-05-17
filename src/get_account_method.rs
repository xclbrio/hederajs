use failure::Error;
use crate::Client;
use std::env;

pub fn get_account_func<'a>(input_operator: &str, input_node_port: &str, input_node_account: &str, input_private_key: &'static str) -> &'a str  {


    let operator = input_operator.parse().unwrap();
    //let node_port: String = input_node_port.parse().unwrap();
//"testnet.hedera.com:50121"
    let client = Client::builder(input_node_port)
        .node(input_node_account.parse().unwrap())
        .operator(operator, move || input_private_key)
        .build().unwrap();


    // Get _just_ the balance for the account first
    // This costs 100,000 tinybar

    let balance = client.account(operator).balance().get().unwrap();
    println!("balance = {} tinybars", balance);

    // Now actually get the full information for the account
    // This costs 100,000 tinybar

    let info = client.account(operator).info().get().unwrap();
    println!("info = {:#?}", info);


    "Result returned from get_account"
}
