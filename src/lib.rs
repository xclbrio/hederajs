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
mod get_account_method;
mod create_file_from_file_method;
mod create_contract_method;
mod call_contract_method;

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
    transaction_record::{TransactionRecord},
    get_account_method::*,
    create_file_from_file_method::*,
    create_contract_method::*,
    call_contract_method::*,
};

use once_cell::{sync::Lazy, sync_lazy};
use parking_lot::Mutex;
use tokio::runtime::Runtime;

// Used to provide a blocking API for Query and Transaction execution
static RUNTIME: Lazy<Mutex<Runtime>> = sync_lazy! { Mutex::new(Runtime::new().unwrap()) };


//=======================================================
// Importing crate for converting a string from UTF-16 (JavaScript(Browser) and JavaScript(Node.js)) to UTF-8 (Rust) for use in your project
extern crate libc;

// Connecting libraries to convert strings
use std::ffi::CStr;
use std::ffi::CString;
use std::str;
use libc::c_char;


// Pointer (*const c_char) initialization declaration
static mut INITIALIZED_POINTER: bool = false;


// This function allow you to specify string in UTF-16 format and convert this Unicode representation to UTF-8 representation for Rust
extern "C" fn convert_to_UTF_8<'a>(input_string: *const c_char) -> &'a str {

    unsafe {
        if !INITIALIZED_POINTER {
            //println!("Initialization this function call");
            INITIALIZED_POINTER = true;
        }
    }

    let string_pointer = unsafe {
        CStr::from_ptr(input_string)
    };

    let string_value = string_pointer.to_str().unwrap();
    string_value

}


// This function allow you to specify string in UTF-8 format and convert this Unicode representation to UTF-16 representation for JavaScript(Browser) and JavaScript(Node.js)
extern "C" fn convert_to_UTF_16(input_string: &str) -> *const c_char {

    let string_value = CString::new(input_string.as_bytes()).unwrap();

    string_value.into_raw()

}


// Create Object representation from N number of *const
fn create_object_representation(match_key: i32, input_strings: &[&str]) -> *const c_char {

    let mut strings_to_object_representation: String = "".to_string();

    match match_key {
        2 => {
            strings_to_object_representation = format!(r#"
			{{
				"firstKey": {},
				"secondKey": {},
			}}"#, input_strings[0], input_strings[1]);
        },
        3 => {
            strings_to_object_representation = format!(r#"
			{{
				"firstKey": {},
				"secondKey": {},
				"thirdKey": {},
			}}"#, input_strings[0], input_strings[1], input_strings[2]);
        },
        4 => {
            strings_to_object_representation = format!(r#"
			{{
				"firstKey": {},
				"secondKey": {},
				"thirdKey": {},
				"fourthKey": {},
			}}"#, input_strings[0], input_strings[1], input_strings[2], input_strings[3]);
        },
        5 => {
            strings_to_object_representation = format!(r#"
			{{
				"firstKey": {},
				"secondKey": {},
				"thirdKey": {},
				"fourthKey": {},
				"fifthKey": {},
			}}"#, input_strings[0], input_strings[1], input_strings[2], input_strings[3], input_strings[4]);
        },
        _ => {},
    }

    let string_value = CString::new(strings_to_object_representation.as_bytes()).unwrap();

    string_value.into_raw()

}


// comment
#[no_mangle]
pub extern fn get_account(input_operator: *const c_char, input_node_port: *const c_char, input_node_account: *const c_char, input_private_key: *const c_char) -> *const c_char {
    convert_to_UTF_16(get_account_func(convert_to_UTF_8(input_operator),convert_to_UTF_8(input_node_port),convert_to_UTF_8(input_node_account),convert_to_UTF_8(input_private_key)))
}

#[no_mangle]
pub extern fn create_file_from_file(input_operator: *const c_char, input_node_port: *const c_char, input_node_account: *const c_char, input_private_key: *const c_char, input_file_path: *const c_char) -> *const c_char {
    convert_to_UTF_16(create_file_from_file_func(convert_to_UTF_8(input_operator),convert_to_UTF_8(input_node_port),convert_to_UTF_8(input_node_account),convert_to_UTF_8(input_private_key),convert_to_UTF_8(input_file_path)))
}

#[no_mangle]
pub extern fn create_contract(input_operator: *const c_char, input_node_port: *const c_char, input_node_account: *const c_char, input_private_key: *const c_char, input_file_id: *const c_char, input_gas: *const c_char) -> *const c_char {
    convert_to_UTF_16(create_contract_func(convert_to_UTF_8(input_operator),convert_to_UTF_8(input_node_port),convert_to_UTF_8(input_node_account),convert_to_UTF_8(input_private_key),convert_to_UTF_8(input_file_id),convert_to_UTF_8(input_gas)))
}

#[no_mangle]
pub extern fn call_contract(input_operator: *const c_char, input_node_port: *const c_char, input_node_account: *const c_char, input_private_key: *const c_char, input_contract_id: *const c_char, input_gas: *const c_char, input_abi_path: *const c_char, input_function: *const c_char) -> *const c_char {
    convert_to_UTF_16(call_contract_func(convert_to_UTF_8(input_operator),convert_to_UTF_8(input_node_port),convert_to_UTF_8(input_node_account),convert_to_UTF_8(input_private_key),convert_to_UTF_8(input_contract_id),convert_to_UTF_8(input_gas),convert_to_UTF_8(input_abi_path),convert_to_UTF_8(input_function)))
}
