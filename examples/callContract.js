const Excalibur_ = require("../JavaScript/Excalibur.js");

// set node settings
const nodeAddress = "t1.hedera.com:50000";
const nodeAccount = "0.0.3";
const excalibur = new Excalibur_(nodeAddress, nodeAccount);

// set account settings
const userAccount = "0.0.1000";
const userPrivateKey = "***";

// contract settings
const contractID = "0.0.1096";
const gasValue = "1000000";
const pathToAbi = "smartContracts/HelloWorld.abi";
const methodName = "getInt";
const arguments = "15,21";

// call smart contract method
console.log(excalibur.callContract(userAccount, userPrivateKey, contractID, gasValue, pathToAbi, methodName, arguments));
