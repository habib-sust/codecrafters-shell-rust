use core::str;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Wait for user input
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        match input.trim() {
            "exit 0" => std::process::exit(0),
            command => print_echo(command),
        };
    }
}

fn print_echo(command: &str) {
    // match command.strip_prefix("echo") {
    //     Some(reminder) => println!("{}", reminder.trim()),
    //     None => println!("echo command not found"),
    // };

    // Using if let
    if let Some(reminder) = command.strip_prefix("echo") {
        println!("{}", reminder.trim())
    }
}
