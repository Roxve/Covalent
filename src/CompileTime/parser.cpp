#include "parser.hpp"
#include <iostream>

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
