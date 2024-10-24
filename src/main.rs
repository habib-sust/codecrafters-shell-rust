use core::str;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    // Wait for user input
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        handle_command(input.trim());
    }
}

fn handle_command(command: &str) {
    let tokens: Vec<&str> = command.split(' ').collect();

    // Slice
    match tokens[..] {
        ["exit", code] => exit_with_code(code),
        ["echo", ..] => println!("{}", tokens[1..].join(" ")),
        _ => println!("{}: command not found", command),
    }
}

fn exit_with_code(code: &str) {
    let code = code.parse::<i32>().unwrap();
    std::process::exit(code);
}
