use std::io::{self, Write};

pub fn run_editor() {
    println!("Simple Rust Editor - Type your text:");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    println!("You typed: {}", buffer);
}