use core::str;
use std::path::PathBuf;
use std::process::Command;
use std::{collections::HashSet, env};

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
        ["pwd"] if tokens.len() == 1 => handle_pwd_command(command),
        ["pwd", ..] => command_not_found(command),
        ["exit", code] => exit_with_code(code),
        ["echo", ..] => handle_echo_command(&tokens[1..].join(" ")),
        ["type", cmd] => handle_type_command(cmd),
        ["cd", path] => handle_cd_command(path),
        ["cat", ..] => hanlde_cat_command(&tokens[1..].join(" ")),
        _ => handle_external_run(command),
    }
}

fn hanlde_cat_command(s: &str) {
    let file_names = &extract_quoted_string(s);
    let mut output = Vec::new();
    for file in file_names {
        let content = std::fs::read_to_string(file)
            .expect(&format!("unable to read from file name: {}", file))
            .replace("\n", "");
        output.push(content);
    }

    println!("{}", output.join(""));
}

fn extract_quoted_string(s: &str) -> Vec<String> {
    s.split(['\'', '"'])
        .filter_map(|part| {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        })
        .collect()
}
fn handle_echo_command(s: &str) {
    if is_surrounded_by_single_quote(s) {
        println!("{}", remove_single_quote(s));
    } else if is_surrounded_by_double_quote(s) {
        println!("{}", remove_double_quote(s));
    } else {
        println!("{}", s.split_whitespace().collect::<Vec<&str>>().join(" "));
    }
}
fn is_surrounded_by_single_quote(s: &str) -> bool {
    s.starts_with('\'') && s.ends_with('\'')
}

fn is_surrounded_by_double_quote(s: &str) -> bool {
    s.starts_with('"') && s.ends_with('"')
}

fn remove_single_quote(s: &str) -> String {
    s.split('\'')
        .filter_map(|part| {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        })
        .collect::<Vec<&str>>()
        .join(" ")
}

fn remove_double_quote(s: &str) -> String {
    s.split('"')
        .filter_map(|part| {
            let trimmed = part.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        })
        .collect::<Vec<&str>>()
        .join(" ")
}

fn handle_cd_command(path: &str) {
    let new_dir = if path == "~" {
        PathBuf::from(env::var("HOME").unwrap_or_default())
    } else {
        PathBuf::from(path)
    };

    match env::set_current_dir(&new_dir) {
        Ok(_) => {
            // if let Ok(current_dir) = env::current_dir() {
            //     println!("{}", current_dir.display());
            // }
        }
        Err(_) => println!("cd: {}: No such file or directory", path),
    }
}

fn handle_pwd_command(command: &str) {
    let directory = env::current_dir();

    match directory {
        Ok(result) => println!("{}", result.display()),
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
