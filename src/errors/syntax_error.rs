use std::fmt;

#[derive(Clone, Debug)]
pub struct SyntaxError {
    line: i32,
    message: String,
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Compilation Error: {}\nAt Line: {}",
            self.message, self.line
        )
    }
}
