use std::env;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Usage: ./htpasswd <filename> <username>");
    }
    
    let filename: &Path = Path::new(&args[1]);
    let username: &str = &args[2];
    
    //TODO: Manage hash
    let password: &str = "test";

    let mut file: File = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .append(true)
    .open(filename)
    .expect(&format!("Cannot open file {}", filename.display()));

    file.write_all(
        String::from(format!("{}:{}", username, password))
        .as_bytes()
    ).expect("Cannot write your credentials.");

}
