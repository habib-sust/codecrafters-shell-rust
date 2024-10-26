use core::str;
use std::collections::HashSet;

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

        handle_command(input.trim());
    }
}

fn handle_command(command: &str) {
    let tokens: Vec<&str> = command.split(' ').collect();

    // Slice
    match tokens[..] {
        ["exit", code] => exit_with_code(code),
        ["echo", ..] => println!("{}", tokens[1..].join(" ")),
        ["type", cmd] => handle_type_command(cmd),
        _ => println!("{}: command not found", command),
    }
}

fn handle_type_command(command: &str) {
    let shell_builtin_commands = HashSet::from(["echo", "exit", "type"]);
    let path_env = std::env::var("PATH").unwrap();

    if shell_builtin_commands.contains(command) {
        println!("{} is a shell builtin", command)
    } else {
        // println!("{}: not found", command)
        let splits = &mut path_env.split(":");

        if let Some(path) =
            splits.find(|path| std::fs::metadata(format!("{}/{}", path, command)).is_ok())
        {
            println!("{} is {}", command, path);
        } else {
            println!("{}: not found", command)
        }
    }
}

fn exit_with_code(code: &str) {
    let code = code.parse::<i32>().unwrap();
    std::process::exit(code);
}
