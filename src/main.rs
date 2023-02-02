use std::fs::File;
use std::io::stdin;
use std::io::stdout;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process;

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

fn run(input: String) {
    let tokens = input.split(' ');
    print!("> Tokens:");
    for t in tokens {
        print!("\n{}", t);
    }
}

fn run_prompt() {
    let mut user_input = String::new();
    let stdin = stdin();
    let mut stdout = stdout();
    loop {
        print!("> ");
        stdout.flush().unwrap();
        match stdin.read_line(&mut user_input) {
            Ok(_) => {
                run(user_input.clone());
                user_input.clear();
            }
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
            Ok(_) => {
                run(file_contents);
            }
            Err(_) => println!("Failed to read file: {}", f_name),
        },
        Err(_) => println!("Failed to read file: {}", f_name),
    }
}
