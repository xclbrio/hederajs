const ffi = require("ffi");
const UTF = require("utf8");

const lib2 = ffi.Library('../target/debug/' + 'libhedera.so', {
    get_account: ["string", ["string", "string", "string", "string"]],
    create_file_from_file: ["string", ["string", "string", "string", "string", "string"]],
    create_contract: ["string", ["string", "string", "string", "string", "string", "string"]],
    call_contract: ["string", ["string", "string", "string", "string", "string", "string", "string", "string"]],
});
