use crate::sha2::Digest;

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
