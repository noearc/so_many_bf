use crate::parser::tokenize;
use crate::token::Token;
use crate::token::Token::*;
use std::collections::HashMap;
use std::io::prelude::*;

type DLoc = i32;
type ILoc = usize;
type DVal = i32;

pub struct Program {
    data_pointer: DLoc,
    instruction_pointer: ILoc,
    data_tape: HashMap<DLoc, DVal>,
    instruction_list: Vec<Token>,
}

impl Program {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            data_pointer: 0,
            instruction_pointer: 0,
            data_tape: HashMap::new(),
            instruction_list: tokens,
        }
    }

    pub fn compile(src: &str) -> Self {
        let tokens = tokenize(src);
        Program::new(tokens)
    }
}

pub fn run(mut program: Program) {
    while !program.is_at_end() {
        let instruction = program.current_instruction();
        match instruction {
            MoveRight => program.inc_data_pointer(),
            MoveLeft => program.dec_data_pointer(),
            Increment => program.inc_data_field(),
            Decrement => program.dec_data_field(),
            Print => print!("{}", program.current_ascii()),
            Read => {
                let input = read_char();
                program.put_data(input as i32);
            }
            OpenLoop => {
                let current_value = program.current_value();
                if current_value == 0 {
                    program.jump_to_closing();
                }
            }
            CloseLoop => {
                let current_value = program.current_value();
                if current_value != 0 {
                    program.jump_back_to_open();
                }
            }
            Comment(_) => (),
        }
        program.inc_inst_pointer();
    }
}

fn read_char() -> i32 {
    let input: Option<i32> = std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as i32);

    match input {
        Some(n) => n,
        None => panic!("Read from stdin failed"),
    }
}

impl Program {
    pub fn is_at_end(&self) -> bool {
        self.instruction_pointer >= self.instruction_list.len()
    }

    pub fn current_instruction(&self) -> &Token {
        &self.instruction_list[self.instruction_pointer]
    }

    pub fn current_value(&self) -> i32 {
        let index = self.data_pointer;
        match self.data_tape.get(&index) {
            Some(n) => *n,
            None => 0,
        }
    }

    pub fn inc_data_pointer(&mut self) {
        self.data_pointer += 1;
    }

    pub fn dec_data_pointer(&mut self) {
        self.data_pointer -= 1;
    }

    pub fn inc_data_field(&mut self) {
        let index = self.data_pointer;
        match self.data_tape.get(&index) {
            Some(n) => self.put_data(n + 1),
            None => self.put_data(1),
        }
    }

    pub fn dec_data_field(&mut self) {
        let index = self.data_pointer;
        match self.data_tape.get(&index) {
            Some(n) => self.put_data(n - 1),
            None => self.put_data(-1),
        }
    }
    pub fn inc_inst_pointer(&mut self) {
        self.instruction_pointer += 1;
    }

    pub fn current_ascii(&self) -> char {
        match self.data_tape.get(&self.data_pointer) {
            Some(n) => char::from_u32(*n as u32).expect("Couldn't read current byte"),
            None => char::from_u32(0 as u32).unwrap(),
        }
    }

    pub fn put_data(&mut self, data: i32) {
        let index = self.data_pointer;
        self.data_tape.insert(index, data);
    }

    pub fn jump_to_closing(&mut self) {
        let mut pointer = self.instruction_pointer;
        let mut bracket_count = 1;
        while bracket_count > 0 {
            pointer += 1;
            if self.instruction_list[pointer] == CloseLoop{
                bracket_count -= 1;
            } else if self.instruction_list[pointer] == OpenLoop {
                bracket_count += 1;
            }
        }

        self.instruction_pointer = pointer;
    }

    pub fn jump_back_to_open(&mut self) {
        let mut pointer = self.instruction_pointer;
        let mut bracket_count = -1;
        while bracket_count < 0 {
            pointer -= 1;

            if self.instruction_list[pointer] == CloseLoop {
                bracket_count -= 1;
            } else if self.instruction_list[pointer] == OpenLoop {
                bracket_count += 1;
            }
        }

        self.instruction_pointer = pointer;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::tokenize;

    #[test]
    fn jump_forward() {
        let src = "[+++>><<..]";
        let tokens = tokenize(src);

        let mut program = Program::new(tokens);

        program.jump_to_closing();

        assert_eq!(program.instruction_pointer, src.len() - 1);
    }

    #[test]
    fn jump_backward() {
        let src = "[+++>><<..]";
        let tokens = tokenize(src);

        let mut program = Program::new(tokens);
        program.instruction_pointer = src.len() - 1;

        program.jump_back_to_open();

        assert_eq!(program.instruction_pointer, 0);
    }

    #[test]
    fn jump_forward_nested() {
        let src = "[+++>>[..++--]<<..]";
        let tokens = tokenize(src);

        let mut program = Program::new(tokens);

        program.jump_to_closing();

        assert_eq!(program.instruction_pointer, src.len() - 1);
    }

    #[test]
    fn jump_backward_nested() {
        let src = "[+++>><[+++--]<..]";
        let tokens = tokenize(src);

        let mut program = Program::new(tokens);
        program.instruction_pointer = src.len() - 1;

        program.jump_back_to_open();

        assert_eq!(program.instruction_pointer, 0);
    }
}

