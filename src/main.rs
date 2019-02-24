use std::env;
use std::fs::{File, OpenOptions};
use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::path::Path;

extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash};


fn main() {
    //Collect CLI args to configure the program
    let args: Vec<String> = env::args().collect();

    //Require at least two additional arguments, filename and username.
    if args.len() < 3 {
        panic!("Usage: ./htpasswd <filename> <username>");
    }
    
    let filename: &Path = Path::new(&args[1]);
    let username: &str = &args[2];

    //Read the password from stdin
    let mut password: String = String::new();
    println!("Type password:");
    io::stdin().read_line(&mut password).expect("Cannot read the password fron STDIN");
    
    //Encrypt password using bcrypt
    let encrypted_password = hash(password.trim(), DEFAULT_COST).unwrap();
    
    /*
     * Open the file given in arguments.
     * It's open for read/write, is created if it doesn't exist, and the content
     * is appended if the file already exists.
     */
    let mut file: File = match OpenOptions::new()
   // .read(true)
    .write(true)
    .create(true)
    .append(true)
    .open(filename) {
        Ok(f) => f,
        Err(e) => {
            panic!("Cannot open the file: {}", e.description());
        }
    };

    //Write the content into the file
    file.write_all(
        String::from(format!("\n{}:{}\n", username, encrypted_password))
        .as_bytes()
    ).expect("Cannot write credentials.");

    
}
