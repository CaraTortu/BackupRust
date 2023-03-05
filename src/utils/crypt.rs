use openssl::{base64, symm};
use std::str;

pub fn encrypt(msg: &String, key: &String, nonce: &String) -> Result<String, String> {
    let cipher = symm::Cipher::aes_256_cbc();
    let iv = nonce.as_bytes();

    match symm::encrypt(cipher, key.as_bytes(), Some(iv), msg.as_bytes()) {
        Ok(cipher) => Ok(base64::encode_block(&cipher)),
        Err(e) => Err(format!("[-] Error encrypting: {e}")),
    }
}

pub fn decrypt(msg: &String, key: &String, nonce: &String) -> Result<String, String> {
    let cipher = symm::Cipher::aes_256_cbc();
    let decoded_b64 = base64::decode_block(msg).unwrap();
    let iv = nonce.as_bytes();

    let result = symm::decrypt(cipher, key.as_bytes(), Some(iv), &decoded_b64);

    match result {
        Ok(res) => Ok(str::from_utf8(&res).unwrap().to_owned()),
        Err(e) => Err(format!("[-] Error decrypting: {e}")),
    }
}
