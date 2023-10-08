#include "parser.hpp"
#include <iostream>
#include <vector>
using namespace std;

Parser::Parser(string code) : tokenizer(code) {
  this->code = code;
  this->tokenizer.tokenize();
}

void Parser::update() {
  this->line = this->tokenizer.line;
  this->colmun = this->tokenizer.colmun;
}

Token Parser::take() {
  Token prev = this->tokenizer.current_token;
  this->tokenizer.tokenize();
  this->update();
  return prev;
}

Token Parser::at() {
  this->update();
  return this->tokenizer.current_token;
}

bool Parser::notEOF() {
  return at().type != TokenType::eof;
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
      Num num(stof(at().value), line, colmun);
      expr = &num;
      return expr;
  }
}