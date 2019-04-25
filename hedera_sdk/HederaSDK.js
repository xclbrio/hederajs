// Connect library to use methods from another library using foreign function interface (FFI)
const NodeFFI = require("ffi");

// Import a "package.json" file and it's binding to a constant for reading library version
const Package = require("./package.json");

// Import a configuration file and it's binding to a constant for working with it
const Settings = require("./config/settings.json");


// The `HederaSDK` class constructor which includes methods for working with it
function HederaSDK(address, port, targetOS = "Linux") {
	
	// Reading the path from settings.json depending on the selected OS
	this.libraryPath = Settings[targetOS] ? Settings[targetOS] : console.log(`Target OS ${targetOS} is not define`);
	
	// Creating a variable and storing the address to the hedera hashgraph node in it
	this.address = address;
	// Creating a variable and storing the port to the hedera hashgraph node in it
	this.port = port;
	
	// Importing a dynamic library with functions to work based on the selected OS
	this.importLibrary = NodeFFI.Library(this.libraryPath, {
		"get_account": ["object", ["string", "string", "string", "string"]],
		"generate_key": ["void", []],
		"create_file": ["void", ["string", "string", "string", "string"]],
		"create_file_from_file": ["void", ["string", "string", "string", "string", "string"]],
		"create_account": ["void", ["string", "string", "string", "string"]],
		"append_file": ["void", ["string", "string", "string", "string", "string"]],
		"update_account": ["void", ["string", "string", "string", "string"]],
		"transfer_crypto": ["void", ["string", "string", "string", "string", "string", "string"]],
		"create_contract": ["void", ["string", "string", "string", "string", "string", "string"]],
		"call_contract": ["void", ["string", "string", "string", "string", "string", "string", "string"]]
	});
	
}


// This method allows you to get account information
HederaSDK.prototype.getAccount = function (operator, operatorSecret) {
	return this.importLibrary.get_account(operator, this.address, this.port, operatorSecret);
}


// This method allows you to generate a key
HederaSDK.prototype.generateKey = function () {
	this.importLibrary.generate_key();
}


// This method allows you to create a file
HederaSDK.prototype.createFile = function (operator, operatorSecret) {
	this.importLibrary.create_file(operator, this.address, this.port, operatorSecret);
}


// This method allows you to create a file from another file
HederaSDK.prototype.createFileFromFile = function (operator, operatorSecret, pathToFile) {
	this.importLibrary.create_file_from_file(operator, this.address, this.port, operatorSecret, pathToFile);
}


// This method allows you to create a new account
HederaSDK.prototype.createAccount = function (operator, operatorSecret) {
	this.importLibrary.create_account(operator, this.address, this.port, operatorSecret);
}


// This method allows you to add new information to the file
HederaSDK.prototype.appendFile = function (operator, operatorSecret, pathToFile) {
	this.importLibrary.append_file(operator, this.address, this.port, operatorSecret, pathToFile);
}


// This method allows you to update your account information
HederaSDK.prototype.updateAccount = function (operator, operatorSecret) {
	this.importLibrary.update_account(operator, this.address, this.port, operatorSecret);
}


// This method allows you to send a certain amount of cryptocurrency
HederaSDK.prototype.transferCrypto = function (operator, operatorSecret, receiverSecret, accountID) {
	this.importLibrary.transfer_crypto(operator, this.address, this.port, operatorSecret, receiverSecret, accountID);
}


// This method allows you to create a smart contract
HederaSDK.prototype.createContract = function (operator, operatorSecret, fileID, gas) {
	this.importLibrary.create_contract(operator, this.address, this.port, operatorSecret, fileID, gas);
}


// This method allows you to call a smart contract with the contract id and its ABI
HederaSDK.prototype.callContract = function (operator, operatorSecret, smartContractABI, contractID, gas) {
	this.importLibrary.call_contract(operator, this.address, this.port, operatorSecret, smartContractABI, contractID, gas);
}


// This method allows you to get information about version this JavaScript SDK
HederaSDK.prototype.version = function () {
	console.log(`Hedera SDK JS ver. ${Package.version}`);
}


// Export HederaSDK library for use in other projects
module.exports = HederaSDK;