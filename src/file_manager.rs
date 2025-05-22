use std::fs::{self, File};
use std::io::{self, Write};

pub fn load_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn save_file(path: &str, contents: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())
}