pub mod crypt;
pub mod file;

use rand::prelude::*;
// use std::io::{stdin, stdout, Write};

pub fn help(args: &Vec<String>) -> () {
    println!("Backup Program by Javier Díaz.\n\nMissing arguments. Example usage:\n {} encrypt example.txt\n\nModes:\n- encrypt [key file] [file] => Encrypts a file\n- decrypt [key file] [file] => Decrypts a file\n- generate_key => Generates a secure key to be used\n\nOptional arguments:\n-h => display this message", args[0])
}

/* 

// Python-like input function

pub fn input(query: &str) -> Result<String, String> {
    match stdout().write(query.as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(format!("[-] Error printing to stdout: {e}")),
    };

    match stdout().flush() {
        Ok(_) => (),
        Err(e) => return Err(format!("[-] Error flushing: {e}")),
    };

    let mut input = String::new();

    match stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(e) => return Err(format!("[-] Error getting input: {e}")),
    };

    Ok(input)
} 

*/

pub fn generate_key(len: usize) -> String {
    let d = "abcdefghijkmnopqrstuvwxyzABCDEFGHIJKLMNOPRSTUVWXYZ1234567890_-!$%&"
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<_>>();

    let mut key = String::new();
    let mut thread = rand::thread_rng();

    for _ in 0..len {
        let index = (thread.gen::<f64>() * d.len() as f64).floor() as usize;

        key += &d[index];
    }

    return key;
}

pub fn parse_key(path: &str) -> Result<Vec<String>, String> {
    match file::get_file_contents(path) {
        Ok(c) => Ok(c.split("\n").map(|c| c.to_owned()).collect::<Vec<String>>()),
        Err(e) => return Err(e),
    }
}
