mod token;
mod parser;
mod program;
use std::io;
use crate::program::*;

fn main() {
    println!("Enter your bf code:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let program = Program::compile(&input);
    run(program);
}
