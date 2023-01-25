fn main() {
    let args = std::env::args();
    let args_len = args.count();
    match args_len {
        1 => println!("Starting lox interpreter..."), // `loxrs` is the always the first argument
        2 => println!("Compiling"),                   // accept one script
        _ => println!("Usage: loxrs [script]"),       // ignore more than 1 argument
    }
}
