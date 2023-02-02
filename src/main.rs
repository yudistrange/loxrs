use crate::errors::syntax_error::SyntaxError;

use std::fs::File;
use std::io::stdin;
use std::io::stdout;
use std::io::Read;
use std::io::Write;
use std::path::Path;

mod errors;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let args_len = args.len();
    match args_len {
        1 => {
            println!("Starting lox interpreter...");
            run_prompt();
        } // `loxrs` is the always the first argument
        2 => {
            println!("Compiling");
            run_file(args[1].clone());
        } // accept one script
        _ => println!("Usage: loxrs [script]"), // ignore more than 1 argument
    }
}

fn run(input: String) -> Result<Vec<String>, SyntaxError> {
    let tokens: Vec<String> = input
        .split(' ')
        .map(|s: &str| -> String { s.to_string() })
        .collect::<Vec<String>>();
    Ok(tokens)
}

fn run_prompt() {
    let mut user_input = String::new();
    let stdin = stdin();
    let mut stdout = stdout();
    loop {
        print!("> ");
        stdout.flush().unwrap();
        match stdin.read_line(&mut user_input) {
            Ok(_) => match run(user_input.clone()) {
                Ok(_tokens) => {
                    println!("Compiled successfully");
                    user_input.clear();
                }
                Err(_) => println!("Error in syntax"),
            },
            Err(_) => {
                println!("Failed to get user input");
                break;
            }
        }
    }
}

fn run_file(f_name: String) {
    let mut file_contents = String::new();
    match File::open(Path::new(f_name.as_str())) {
        Ok(mut f) => match f.read_to_string(&mut file_contents) {
            Ok(_) => match run(file_contents) {
                Ok(_tokens) => println!("Compiled successfully"),
                Err(_) => println!("Compilation failed"),
            },
            Err(_) => println!("Failed to read file: {}", f_name),
        },
        Err(_) => println!("Failed to read file: {}", f_name),
    }
}
