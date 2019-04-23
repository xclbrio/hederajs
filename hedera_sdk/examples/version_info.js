// Connection HederaSDK library for further use
const HederaSDK = require("hederasdkjs");

// Creating an instance of the HederaSDK class:
let hashgraph = new HederaSDK("_address_", "_port_");

// Get information about version this HederaSDK
hederaSDK.version();