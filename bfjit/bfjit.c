#include <stdio.h>

typedef enum {
  OP_INC = '+',
  OP_DEC = '-',
  OP_LEFT = '<',
  OP_RIGHT = '>',
  OP_INPUT = '.',
  OP_OUTPUT = ',',
  OP_JUMP_IF_ZERO = '[',
  OP_JUMP_IF_NONZERO = ']',
} Op_Kind;

typedef struct {
  Op_Kind kind;
  size_t operand;
} Op;

typedef struct {
  Op *items;
  size_t count;
  size_t capacity;
}

typedef struct {
    Nob_String_View content;
    size_t pos;
} Lexer;


bool is_bf_cmd(char ch)
{
  const char *cmds = "+-<>,.[]";
  return strchr(cmds, ch) != NULL;
}

int main(void)
{
  Ops ops = { 0 };
  return 0;
}
