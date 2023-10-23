use std::io::{self, Read};

fn main() {
    let input = read_input();
    let tape = bf(&input);
    println!("{:?}", tape);
}

fn read_input() -> Vec<char> {
    println!("Write your Brainfuck code:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.chars().collect()
}

fn bf(input: &[char]) -> Vec<u8> {
    let mut tape = vec![0_u8; 30000];
    let mut tape_ptr = 0_usize;
    let mut output = Vec::new();
    let mut loop_stack = Vec::new();
    let mut instruction_ptr = 0_usize;

    while instruction_ptr < input.len() {
        match input[instruction_ptr] {
            '>' => tape_ptr += 1,
            '<' => tape_ptr -= 1,
            '+' => tape[tape_ptr] = tape[tape_ptr].wrapping_add(1),
            '-' => tape[tape_ptr] = tape[tape_ptr].wrapping_sub(1),
            '.' => {
                let value = tape[tape_ptr];
                print!("{}", value as char);
                output.push(value);
            }
            ',' => {
                let mut input_buffer = [0_u8; 1];
                io::stdin().read_exact(&mut input_buffer).unwrap();
                tape[tape_ptr] = input_buffer[0];
            }
            '[' => {
                if tape[tape_ptr] == 0 {
                    let mut loop_count = 1;
                    while loop_count > 0 {
                        instruction_ptr += 1;
                        match input[instruction_ptr] {
                            '[' => loop_count += 1,
                            ']' => loop_count -= 1,
                            _ => (),
                        }
                    }
                } else {
                    loop_stack.push(instruction_ptr);
                }
            }
            ']' => {
                if tape[tape_ptr] != 0 {
                    instruction_ptr = *loop_stack.last().unwrap() - 1;
                } else {
                    loop_stack.pop();
                }
            }
            _ => (),
        }
        instruction_ptr += 1;
    }
    output
}
