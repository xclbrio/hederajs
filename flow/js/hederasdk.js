// @flow

// Connecting library to use foreign functions
const NodeFFI = require("ffi");


function HederaSDK(selectedTargetOS = 0) {
	
	// Importing a dynamic library with functions to work based on the selected OS
	switch (selectedTargetOS) {
		case 0:
			var libraryPath = "./dynamic_lib/Linux/lib.so";
			break;
		case 1:
			var libraryPath = "./dynamic_lib/macOS/lib.dylib";
			break;
		case 2:
			var libraryPath = "./dynamic_lib/Windows/lib.dll";
			break;
	}
	
	let importLibrary = NodeFFI.Library(libraryPath, {
		// It is a place for functions from dynamic library
		// Example
		/*
		 * "function_name": ["result_type", ["input_type1", ["input_type2"]]]
		 */
	});
	
}


HederaSDK.prototype.getFile = function (/* _ */) {
	// paste code here...
}


module.exports = HederaSDK;