#include "parser.hpp"
#include <iostream>
#include <vector>
using namespace std;

Parser::Parser(string code) : tokenizer(code){
  this->code = code;
  this->tokenizer = Tokenizer(code);
  this->tokenizer.tokenize();
}

void Parser::update() {
  line = this->tokenizer.line;
  colmun = this->tokenizer.colmun;
}

Token Parser::take() {
  Token prev = tokenizer.current_token;
  this->tokenizer.tokenize();
  this->update();
  return prev;
}

Token Parser::at() {
  this->update();
  return tokenizer.current_token;
}

bool Parser::notEOF() {
  cout << "trying.." << endl;
  cout << "line: " << this->at().line << "colmun: " << this->at().colmun << " type: " << this->at().type << " val: " << this->at().value;
  return this->at().type != TokenType::eof;
}

Program Parser::productAST() {
  Program prog = Program(line, colmun);
  while(notEOF()) {
    prog.body.push_back(parse_expr());
  }
  return prog;
}

Expr* Parser::parse_expr() {
  return parse_primary_expr();
}

Expr* Parser::parse_primary_expr() {
  Expr *expr;
  switch(at().type) {
    case TokenType::number :
      string val = this->take().value;
      Num num(stof(val), line, colmun);
      expr = &num;
      return expr;
  }
}
