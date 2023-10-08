#include <iostream>
#include "AST.hpp"
#include "Tokenizer.hpp"

using namespace std;


class Parser {
  private:
    Tokenizer tokenizer;
    Token take();

    Expr parse_expr();
    Expr parse_primary_expr();
  public:
    string code;
    int line;
    int colmun;

    void update();
    Token at();
    Program productAST();

    Parser(string code);
};
