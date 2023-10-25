#! /usr/bin/env ruby
# need command line input

tape = Array.new(3,0)
tape_ptr = 0
program = gets.chomp.chars
program_ptr = 0

while program_ptr <= program.length do
  char = program[program_ptr];
  case char
  when '+'
    tape[tape_ptr] += 1;
  when '-'
    tape[tape_ptr] -= 1;
  when '>'
    tape_ptr += 1;
  when '<'
    tape_ptr -= 1;
  when '.'
    print tape[tape_ptr].chr;
  when ','
    tape[tape_ptr] = STDIN.getc;
  when '['
    if tape[tape_ptr] == 0
      while program[program_ptr] != ']'
        program_ptr += 1;
      end
    end
  when ']'
    if tape[tape_ptr] != 0
      while program[program_ptr] != '['
        program_ptr -= 1;
      end
    end
  end
  program_ptr += 1;
end
