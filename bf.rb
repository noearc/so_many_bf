#! /usr/bin/env ruby
# need command line input

tape = Array.new(3,0)
tape_ptr = 0
program = gets.chomp.chars
program_ptr = 0

while program_ptr <= program.length do
  char = program[program_ptr];
  if char == '+'
    tape[tape_ptr] += 1;
  elsif char == '-'
    tape[tape_ptr] -= 1;
  elsif char == '>'
    tape_ptr += 1;
  elsif char == '<'
    tape_ptr -= 1;
  elsif char == '.'
    print tape[tape_ptr].chr;
  elsif char == ','
    tape[tape_ptr] = STDIN.getc;
  elsif char == '['
    if tape[tape_ptr] == 0
      while program[program_ptr] != ']'
      program_ptr += 1;
      end
    end
  elsif char == ']'
    if tape[tape_ptr] != 0
      while program[program_ptr] != '['
      program_ptr -= 1;
      end
    end
  end
  program_ptr += 1;
end
