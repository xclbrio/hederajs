use hedera::SecretKey;

fn output_generate_key() {
    let (secret, mnemonic) = SecretKey::generate("");
    let public = secret.public();

    println!("secret   = {}", secret);
    println!("mnemonic = {}", mnemonic);
    println!("public   = {}", public);
}
