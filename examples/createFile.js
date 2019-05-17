const Excalibur_ = require("../JavaScript/Excalibur.js");

// set node settings
const nodeAddress = "t1.hedera.com:50000";
const nodeAccount = "0.0.3";
const excalibur = new Excalibur_(nodeAddress, nodeAccount);

// set account settings
const userAccount = "0.0.1000";
const userPrivateKey = "***";

// file settings
const pathToFile = "smartContracts/HelloWorld.bin"; // SC byte code

// create file
console.log(excalibur.createFileFromFile(userAccount, userPrivateKey, pathToFile));
