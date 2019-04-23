// Connection HederaSDK library for further use
const HederaSDK = require("hederasdkjs");

// Creating an instance of the HederaSDK class:
let hashgraph = new HederaSDK("_address_", "_port_");

// Call method and get information about your account
console.log(hederaSDK.getAccount("_oprator_value_", "_private_key_"));