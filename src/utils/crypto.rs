use sha2::Digest;
use crate::utils::*;
use crate::types::*;
use sha1::{Sha1, Digest as sDigest};
use base64::encode;
use hex::encode as encode_hex;
use mysql::*;
use mysql::prelude::*;

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

pub fn hash_salt(mut base: String, salt: &str) -> String {
    base.push_str(salt);

    let mut hasher = Sha1::new();
    hasher.update(base);

    let hashed = hasher.finalize();
    let hashed: &[u8] = hashed.as_slice();
    encode_hex(hashed)
}

pub fn hash_pack(ids: Vec<i32>) -> String {

    let mut hash: String = String::new();

    for id in ids {

        let pack: Row = sql::CONN.lock().unwrap().exec_first(format!("SELECT * FROM mappacks WHERE packID=:id_num"),
            mysql::params! {
                "id_num" => id,
            }
        ).unwrap().unwrap();

        let pack: level_pack::MapPack = level_pack::MapPack::from_row(pack);

        let id_str: String = id.to_string();
        let id_str: &[u8] = id_str.as_bytes();

        hash.push(id_str[0] as char);
        hash.push(id_str[id_str.len() - 1] as char);
        hash.push_str(&pack.stars.to_string());
        hash.push_str(&pack.coins.to_string());

    }

    hash.push_str("xI25fpAapCQg");

    println!("{}", hash);

    let mut hasher = Sha1::new();
    hasher.update(hash);

    let hashed = hasher.finalize();
    let hashed: &[u8] = hashed.as_slice();

    encode_hex(hashed)
}