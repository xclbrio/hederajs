const NodeFFI = require("ffi");

let importLibrary = NodeFFI.Library("./example/target/release/libexample.dylib", {
	"fibonacci_function": ["void", ["int"]]
});

importLibrary.fibonacci_function(3);
