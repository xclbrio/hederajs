var importObject = {
	imports: {
		imported_func: arg => console.log(arg)
	}
}


WebAssembly.instantiateStreaming('number_adder.wasm').then(kek => {
	console.log(kek);
	//console.log(kek.instance.exports.add_numbers(10, 15));
});
