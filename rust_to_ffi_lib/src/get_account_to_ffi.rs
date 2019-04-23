use failure::Error;
use hedera::Client;
use std::env;

fn output_get_account(input_operator: &str, input_address: &str, input_port: &str, input_operator_secret: &str) -> Result<(), Error> {
    //pretty_env_logger::try_init()?;

    let operator = input_operator?.parse()?;
    let client = Client::builder(format!("{}-{}-{}", input_address, ":", input_port))
        .node("0:0:3".parse()?)
        .operator(operator, || input_operator_secret)
        .build()?;

    // Get _just_ the balance for the account first
    // This costs 100,000 tinybar

    let balance = client.account(operator).balance().get()?;
    println!("balance = {} tinybars", balance);

    // Now actually get the full information for the account
    // This costs 100,000 tinybar

    let info = client.account(operator).info().get()?;
    println!("info = {:#?}", info);

    Ok(())
}