const NodeFFI = require("ffi");

let importLibrary = NodeFFI.Library("./embed/target/release/libshrek.dylib", {
	"shrek": ["void", ["int"]]	
});

importLibrary.shrek(3);
