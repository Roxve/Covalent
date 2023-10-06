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
  char prev = code[0];
  code.erase(0, 1);
  return prev;
}

char Tokenizer::at() {
  if(code.size() <= 0) {
    return ' ';
  }
  return code[0];
}

bool Tokenizer::isNum() {
  string nums = "0123456789";
  // find() returns npos if not found
  return nums.find(this->at()) != string::npos;
}


vector<Token> Tokenizer::tokenize() {
  vector<Token> tokens;
  while(code.size() > 0) {
    switch(this->at()) {
      //skippable chars
      case ' ':
      case '\t':
        this->take();
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
        string res;
        while(isNum()) {
          res += this->take();
        }
        tokens.push_back(Token(res, TokenType::number, line, colmun));
        continue;
    }
  }
  tokens.push_back(Token("EOF", TokenType::eof,line, colmun));
  return tokens;
}
