// Connection HederaSDK library for further use
const HederaSDK = require("../HederaSDK.js");

// Creating an instance of the HederaSDK class
let hederaHashgraph = new HederaSDK("_address_", "_port_");

// Call method and transfer a few cryptocurrency
hederaHashgraph.transferCrypto("_operator_value_", "_private_key_", "_receiver_private_key_", "_account_id_");