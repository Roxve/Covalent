#include <iostream>
#include <vector>

enum TokenType {
  ooperator,
  id,
  number,
  null,
  eof
};

struct Token {
  std::string value;
  TokenType type;
  int line;
  int colmun;

  Token(std::string value, TokenType type, int line, int colmun);
};

class Tokenizer {
  private: 
    Token set(std::string value, TokenType type);
    char take();
    char at();
    bool isNum();
    bool isOp();
  public: 
    int line;
    int colmun;

    Token tokenize();
    std::string code;
    Token* current_token;

    Tokenizer(std::string code);
};
