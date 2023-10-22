local program = io.read()
local program_table = {}
for i = 1, #program do
    program_table[i] = program:sub(i, i)
end
local tape = {0}
local cell_index = 1

local loop_table = {}
local loop_stack = {}


local ip = 1

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

while ip <= #program do
  local instrunction  = program_table[ip]
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
      ip = loop_table[ip]
    end
  elseif instrunction == ']' then
    if tape[cell_index] ~= 0 then
      ip = loop_table[ip]
    end
  end
  ip = ip + 1
end
