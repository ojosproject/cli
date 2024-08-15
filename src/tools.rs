use std::io::stdin;

pub fn input(message: &str) -> String {
    let mut input = String::new();

    println!("{}", message);
    stdin().read_line(&mut input).expect("Failed to read input.");

    String::from(input.trim())
}
