use std::env;
use std::path::Path;
use std::fs;
use std::io;
use std::io::Write;
pub mod token;
pub mod token_type;
pub mod scanner;
pub mod interpreter;

fn main() {
    let mut panther = interpreter::Interpreter::new();
    panther.get_run_type();
}
