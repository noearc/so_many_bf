#! /usr/bin/lua

-- problem with space?

print("Write your bf code: \n")

local program = io.read()
local program_table = {}
local program_index = 1
local tape = {0}
local cell_index = 1
local loop_table = {}
local loop_stack = {}


for i = 1, #program do
    program_table[i] = program:sub(i, i)
end

for i = 1, #program_table do
  local instruction = program_table[i]
  if instruction == '[' then
    table.insert(loop_stack, i)
  elseif instruction == ']' then
    local loop_start = table.remove(loop_stack)
    loop_table[i] = loop_start
    loop_table[loop_start] = i
  end
end

while program_index <= #program do
  local instrunction  = program_table[program_index]
  if instrunction == '>' then
    cell_index = cell_index + 1
    if cell_index > #tape then
      table.insert(tape, 0)
    end
  elseif instrunction == '<' then
    cell_index = cell_index - 1
    if cell_index < 1 then
      error("Cell index out of range")
    end
  elseif instrunction == '+' then
    tape[cell_index] = tape[cell_index] + 1
  elseif instrunction == '-' then
    tape[cell_index] = tape[cell_index] - 1
  elseif instrunction == '.' then
    io.write(string.char(tape[cell_index]))
  elseif instrunction == ',' then
    local input = io.read(1)
    tape[cell_index] = string.byte(input)
  elseif instrunction == '[' then
    if tape[cell_index] == 0 then
      program_index = loop_table[program_index]
    end
  elseif instrunction == ']' then
    if tape[cell_index] ~= 0 then
      program_index = loop_table[program_index]
    end
  end
  program_index = program_index + 1
end
