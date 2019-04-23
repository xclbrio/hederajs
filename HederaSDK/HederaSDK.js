// Connecting library to use foreign functions
const NodeFFI = require("ffi");

// Importing Hedera SDK JS version from 'package.json'
const Package = require("./package.json");

// Import target OS from 'settings.json'
const Settings = require("./settings.json");


function HederaSDK(address, port, selectedTargetOS = "Linux") {
	
	// Importing a dynamic library with functions to work based on the selected OS
	this.libraryPath = Settings[selectedTargetOS] ? Settings[selectedTargetOS] : console.log(`Target OS ${selectedTargetOS} is not define`);
	
	this.address = address;
	this.port = port;
	
	this.importLibrary = NodeFFI.Library(this.libraryPath, {
		"get_account": ["object", ["string", "string", "string", "string"]],
		"generate_key": ["void", []],
		"create_file": ["void", ["string", "string", "string", "string"]],
		"create_file_from_file": ["void", ["string", "string", "string", "string", "string"]],
		"create_account": ["void", ["string", "string", "string", "string"]],
		"append_file": ["void", ["string", "string", "string", "string", "string"]],
		"update_account": ["void", ["string", "string", "string", "string"]],
		"transfer_crypto": ["void", ["string", "string", "string", "string", "string", "string"]]
	});
	
}


// Get Account
HederaSDK.prototype.getAccount = function (operator, operatorSecret) {
	return this.importLibrary.get_account(operator, this.address, this.port, operatorSecret);
}


// Generate Key
HederaSDK.prototype.generateKey = function () {
	this.importLibrary.generate_key();
}


// Create File
HederaSDK.prototype.createFile = function (operator, operatorSecret) {
	this.importLibrary.create_file(operator, this.address, this.port, operatorSecret);
}


// Create File From File
HederaSDK.prototype.createFileFromFile = function (operator, operatorSecret, pathToFile) {
	this.importLibrary.create_file_from_file(operator, this.address, this.port, operatorSecret, pathToFile);
}


// Create Account
HederaSDK.prototype.createAccount = function (operator, operatorSecret) {
	this.importLibrary.create_account(operator, this.address, this.port, operatorSecret);
}


// Append file
HederaSDK.prototype.appendFile = function (operator, operatorSecret, pathToFile) {
	this.importLibrary.append_file(operator, this.address, this.port, operatorSecret, pathToFile);
}


// Update Account
HederaSDK.prototype.updateAccount = function (operator, operatorSecret) {
	this.importLibrary.update_account(operator, this.address, this.port, operatorSecret);
}


// Transfer Crypto
HederaSDK.prototype.transferCrypto = function (operator, operatorSecret, receiverSecret, accountID) {
	this.importLibrary.transfer_crypto(operator, this.address, this.port, operatorSecret, receiverSecret, accountID);
}


// Get Information About Version This SDK
HederaSDK.prototype.version = function () {
	console.log(`Hedera SDK JS ver. ${Package.version}`);
}


module.exports = HederaSDK;