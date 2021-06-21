use crate::sha2::Digest;
use sha1::{Sha1, Digest as sDigest};
use base64::encode;
use hex::encode as encode_hex;

/// Hash a message with salt
/// 
/// * `message` - The message to hash
/// * `salt` - The salt to hash with
pub fn hash(message: String, salt: String) -> String {

    let mut sha256 = sha2::Sha256::new();

    sha256.input(salt.as_bytes());
    sha256.input(message.as_bytes());

    format!("{:X}", sha256.result())

}

pub fn generate_chk(mut values: Vec<String>, key: &str, salt: &str) -> String {
    values.push(salt.to_string());

    let value_str: String = values.join("");

    let mut hasher = Sha1::new();
    hasher.update(value_str);

    let hashed = hasher.finalize();
    let hashed: &[u8] = hashed.as_slice();
    let xored: String = xor_cipher(encode_hex(hashed).as_bytes(), key.as_bytes()).to_string();
    
    encode(xored)
}

pub fn xor_cipher(string: &[u8], key: &[u8]) -> String {
    let mut result: String = String::new();

    for i in 0..string.len() {
        let c = string[i] ^ key[i % key.len()];
        result.push(c as char);
    }

    result
}