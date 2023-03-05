use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn get_file_contents(path: &str) -> Result<String, String> {
    if !path_exists(path) {
        return Err("[-] Path does not exist".to_owned());
    }

    // We can unwrap since we know that the path exists
    let f: File = File::open(path).unwrap();
    let mut reader: BufReader<File> = BufReader::new(f);
    let mut buf: String = String::new();

    let result = reader.read_to_string(&mut buf);

    match result {
        Ok(_) => return Ok(buf),
        Err(e) => return Err(format!("[-] Error reading file: {e}")),
    };
}

pub fn write_file(path: &str, contents: &String) -> Result<(), String> {
    // We can unwrap since we know that the path exists
    let f = File::create(path).unwrap();
    let mut writer = BufWriter::new(f);

    match writer.write(contents.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("[-] Error writing file: {e}")),
    }
}
