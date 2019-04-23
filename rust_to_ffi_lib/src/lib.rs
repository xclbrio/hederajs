#![feature(async_await, await_macro, futures_api)]
#![warn(clippy::pedantic, future_incompatible, unreachable_pub)]
#![allow(clippy::stutter, clippy::new_ret_no_self, clippy::module_inception)]

#[macro_use]
mod macros;

mod claim;
pub mod client;
mod crypto;
mod duration;
mod entity;
mod error;
mod id;
mod info;
mod proto;
pub mod query;
mod status;
mod timestamp;
pub mod transaction;
mod transaction_id;
mod transaction_receipt;
mod transaction_record;


// Adding examples to lib.rs that yoy can compile this library to dynamic lib of your system
use get_account_to_ffi;
use generate_key_to_ffi;
use create_file_to_ffi;
use create_file_from_file_to_ffi;
use create_account_to_ffi;
use append_file_to_ffi;
use update_account_to_ffi;
use transfer_crypto_to_ffi;


pub use self::{
    claim::Claim,
    client::Client,
    crypto::{PublicKey, SecretKey, Signature},
    entity::Entity,
    error::ErrorKind,
    id::*,
    info::{AccountInfo, ContractInfo, FileInfo},
    status::Status,
    transaction_id::TransactionId,
    transaction_receipt::TransactionReceipt,
    transaction_record::{TransactionRecord, TransactionRecordBody},
};

use once_cell::{sync::Lazy, sync_lazy};
use parking_lot::Mutex;
use tokio::runtime::Runtime;

// Used to provide a blocking API for Query and Transaction execution
static RUNTIME: Lazy<Mutex<Runtime>> = sync_lazy! { Mutex::new(Runtime::new().unwrap()) };


// get_account from output_get_account
#[no_mangle]
pub extern fn get_account(input_operator: &str, input_address: &str, input_port: &str, input_operator_secret: &str) -> Result<(), Error> {
    get_account_to_ffi::output_get_account(input_operator, input_address, input_port, input_operator_secret)
}

// generate_key from output_generate_key
#[no_mangle]
pub extern fn generate_key() {
    generate_key_to_ffi::output_generate_key();
}


// create_file from output_create_file
#[no_mangle]
pub extern fn create_file() {
    create_file_to_ffi::output_create_file();
}


// create_file_from_file from output-create_file_from_file
#[no_mangle]
pub extern fn create_file_from_file(input_operator: &str, input_address: &str, input_port: &str, input_operator_secret: &str, input_path_to_file: &str) {
    create_file_from_file_to_ffi::output_create_file_from_file(input_operator, input_address, input_port, input_operator_secret, input_path_to_file);
}


// create_account from output_create_account
#[no_mangle]
pub extern fn create_account(input_operator: &str, input_address: &str, input_port: &str, input_operator_secret: &str) {
    create_account_to_ffi::output_create_account(input_operator, input_address, input_port, input_operator_secret);
}


// append_file from output_append_file
pub extern fn append_file(input_operator: &str, input_address: &str, input_port: &str, input_operator_secret: &str, input_path_to_file: &str) {
    append_file_to_ffi::output_append_file(input_operator, input_address, input_port, input_operator_secret, input_path_to_file);
}


// update_account from output_update_account
pub extern fn update_account(input_operator: &str, input_address: &str, input_port: &str, input_operator_secret: &str) {
    update_account_to_ffi::output_update_account(input_operator, input_address, input_port, input_operator_secret);
}


// transfer_crypto from output_transfer_crypto
pub extern fn transfer_crypto(input_operator: &str, input_address: &str, input_port: &str, input_operator_secret: &str, input_receiver_secret: &str, input_account_id: &str) {
    transfer_crypto_to_ffi::output_transfer_crypto(input_operator, input_address, input_port, input_operator_secret, input_receiver_secret, input_account_id);
}