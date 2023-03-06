use openssl::{base64, symm};
use std::str;

pub fn encrypt(msg: &Vec<u8>, key: &String, nonce: &String) -> Result<String, String> {
    let cipher = symm::Cipher::aes_256_cbc();
    let iv = nonce.as_bytes();

    match symm::encrypt(cipher, key.as_bytes(), Some(iv), msg) {
        Ok(cipher) => Ok(base64::encode_block(&cipher)),
        Err(e) => Err(format!("[-] Error encrypting: {e}")),
    }
}

pub fn decrypt(msg: &Vec<u8>, key: &String, nonce: &String) -> Result<String, String> {
    let cipher = symm::Cipher::aes_256_cbc();
    let decoded_b64 = match base64::decode_block(std::str::from_utf8(msg).unwrap()) {
        Ok(d) => d,
        Err(_) => return Err("[-] The file you are trying to decrypt isnt encrypted using this program".to_owned()) 
    };
    let iv = nonce.as_bytes();

    let result = symm::decrypt(cipher, key.as_bytes(), Some(iv), &decoded_b64);

    match result {
        Ok(res) => Ok(str::from_utf8(&res).unwrap().to_owned()),
        Err(_) => Err(format!("[-] The file you are trying to decrypt is corrupted or the key you used is incorrect")),
    }
}
