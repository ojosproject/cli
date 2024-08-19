use std::{env, io::stdin};

pub fn input(message: &str) -> String {
    let mut input = String::new();

    println!("{}", message);
    stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    String::from(input.trim())
}

pub fn get_home() -> String {
    if env::consts::OS == "windows" {
        env::var("HOMEPATH").unwrap()
    } else {
        env::var("HOME").unwrap()
    }
}
