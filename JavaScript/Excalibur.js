// Import a configuration file and it's binding to a constant for working with it
const Settings = require("./configuration/settings.json");

// Import a "package.json" file and it's binding to a constant for reading library version
const Package = require("./package.json");

// FFI framework import for foreign functions interface ()
const NodeFFI = require("ffi");



// The `Excalibur_` class constructor which includes methods for working with it
function Excalibur_(nodeAddress, nodeAccount, selectedOS = "Linux") {
	
	// Creating a variable and storing the address to the hedera hashgraph node in it
	this.nodeAddress = nodeAddress;
	
	// Creating a variable and storing the port to the hedera hashgraph node in it
	this.nodeAccount = nodeAccount;
	
	// Getting path to the `libhedera` framework
	this.targetOS = (Settings[selectedOS]) ? Settings[selectedOS] : console.log(`Incorrectly selected OS ${selectedOS}...`);
	
	// Importing methods from this framework for working with it
	this.hederaLibrary = NodeFFI.Library(this.targetOS, {
		get_account: ["string", ["string", "string", "string", "string"]],
		create_file_from_file: ["string", ["string", "string", "string", "string", "string"]],
		create_contract: ["string", ["string", "string", "string", "string", "string", "string"]],
		call_contract: ["string", ["string", "string", "string", "string", "string", "string", "string", "string"]],
		get_sdk_version: ["string", []]
	});
	
}



// This method allows you to get information about your current account
Excalibur_.prototype.getAccount = function (userAccount, userPrivateKey) {
	return this.hederaLibrary.get_account(userAccount, this.nodeAddress, this.nodeAccount, userPrivateKey);
}


// This method allows you to create a file from another file by specifying the path to it
Excalibur_.prototype.createFileFromFile = function (userAccount, userPrivateKey, pathToFile) {
	return this.hederaLibrary.create_file_from_file(userAccount, this.nodeAddress, this.nodeAccount, userPrivateKey, pathToFile);
}


// This method allows you to create a contract
Excalibur_.prototype.createContract = function (userAccount, userPrivateKey, fileID, gasValue) {
	return this.hederaLibrary.create_contract(userAccount, this.nodeAddress, this.nodeAccount, userPrivateKey, fileID, gasValue);
}


// This method allows you to call a smart contract using a method from it
Excalibur_.prototype.callContract = function (userAccount, userPrivateKey, contractID, gasValue, pathToABI, methodName) {
	return this.hederaLibrary.call_contract(userAccount, this.nodeAddress, this.nodeAccount, userPrivateKey, contractID, gasValue, pathToABI, methodName);
}


// This method allow you to get information about this framework and hederaLibrary
Excalibur_.prototype.getVersions = function () {
	let Versions = new Object;
	Versions.framework = "Excalibir_ frmwk ver. " + Package.version;
	Versions.library = "Rust Hedera SDK ver. " + this.hederaLibrary.get_sdk_version();
	return Versions;
}



// Export framework for use in other projects
module.exports = Excalibur_;
