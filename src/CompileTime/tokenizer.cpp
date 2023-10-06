#include "Tokenizer.hpp"
#include <iostream>
#include <string>
#include <vector>
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


char Tokenizer::take() {
  if(code.size() <= 0) {
    return ' ';
  }
  code.pop_back();
  return code[0];
}
bool Tokenizer::isNum() {
  return false;
}

vector<Token> Tokenizer::tokenize() {
  vector<Token> tokens;
  while(code.size() > 0) {
    switch(code[0]) {
      //skippable chars
      case ' ':
      case '\t':
        take();
        continue;
      case '0':
      case '1':
      case '2':
      case '3':
      case '4':
      case '5':
      case '6':
      case '7':
      case '8':
      case '9':
        
        continue;
    }
  }
  tokens.push_back(Token("EOF", TokenType::eof,line, colmun));
  return tokens;
}
