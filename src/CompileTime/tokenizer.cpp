#include "Tokenizer.hpp"
#include <iostream>
#include <string>

using namespace std;
Tokenizer::Tokenizer(string code)
  :code(code),
  line(1),
  colmun(0)
{}

Token::Token(string value, TokenType type, int line, int colmun) 
  :value(value), 
  type(type),
  line(line),
  colmun(colmun)
{}



Token Tokenizer::tokenize() {
  return Token("EOF", TokenType::eof,line, colmun);
}
