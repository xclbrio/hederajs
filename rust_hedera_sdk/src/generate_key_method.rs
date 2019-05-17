use crate::SecretKey;

pub fn generate_key_func<'a>() -> &'a str  {
    let (secret, mnemonic) = SecretKey::generate("");
    let public = secret.public();

    println!("secret   = {}", secret);
    println!("mnemonic = {}", mnemonic);
    println!("public   = {}", public);

    "Result returned from get_account"
}
