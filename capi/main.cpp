#include "ellie_c.h"
#include "stdio.h"

int main() {
  uint32_t x = 10;

  ParserOptions p;

  p.break_on_error = true;

  Parsed test = parser_new("d test : S = 10;", p);

  // printf("%d", test.syntax_errors[0]);
}
