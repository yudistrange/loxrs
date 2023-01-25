use std::io::stdin;
use std::io::stdout;
use std::io::Write;

fn main() {
    let args = std::env::args();
    let args_len = args.count();
    match args_len {
        1 => {
            println!("Starting lox interpreter...");
            run_prompt();
        } // `loxrs` is the always the first argument
        2 => println!("Compiling"),             // accept one script
        _ => println!("Usage: loxrs [script]"), // ignore more than 1 argument
    }
}

fn run(input: String) {
    let tokens = input.split(" ");
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
