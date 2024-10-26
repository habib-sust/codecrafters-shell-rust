use core::str;
use std::collections::HashSet;
use std::process::Command;

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

        if input.trim().is_empty() {
            continue;
        }

        handle_command(input.trim());
    }
}

fn handle_command(command: &str) {
    let tokens: Vec<&str> = command.split(' ').collect();

    // Slice
    match tokens[..] {
        ["pwd"] => handle_pwd_command(command),
        ["exit", code] => exit_with_code(code),
        ["echo", ..] => println!("{}", tokens[1..].join(" ")),
        ["type", cmd] => handle_type_command(cmd),
        _ => handle_external_run(command),
    }
}

fn handle_pwd_command(command: &str) {
    let directory = Command::new(command).output();

    match directory {
        Ok(result) => {
            let output = String::from_utf8_lossy(&result.stdout);
            println!("{}", output.trim_end());
        }

        Err(_) => command_not_found(command),
    }
}
fn handle_external_run(command: &str) {
    let commands: Vec<&str> = command.split(' ').collect();

    if commands.is_empty() {
        return;
    }

    let result = if commands.len() > 1 {
        Command::new(commands[0])
            .args(commands[1..].iter())
            .output()
    } else {
        Command::new(commands[0]).output()
    };

    match result {
        Ok(executalbe) => {
            // Trim the output to remove extra whitespace/newlines
            let output = String::from_utf8_lossy(&executalbe.stdout);
            println!("{}", output.trim_end());
        }

        Err(_) => command_not_found(command),
    }
}

fn command_not_found(command: &str) {
    println!("{}: command not found", command);
}

fn handle_type_command(command: &str) {
    let shell_builtin_commands = HashSet::from(["echo", "exit", "type", "pwd"]);
    let path_env = std::env::var("PATH").unwrap();

    if shell_builtin_commands.contains(command) {
        println!("{} is a shell builtin", command)
    } else {
        // println!("{}: not found", command)
        let splits = &mut path_env.split(":");

        if let Some(path) =
            splits.find(|path| std::fs::metadata(format!("{}/{}", path, command)).is_ok())
        {
            println!("{} is {}/{}", command, path, command);
        } else {
            println!("{}: not found", command);
        }
    }
}

fn exit_with_code(code: &str) {
    let code = code.parse::<i32>().unwrap();
    std::process::exit(code);
}
