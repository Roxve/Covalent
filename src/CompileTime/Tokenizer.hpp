#include <iostream>
#include <vector>

enum TokenType {
  ooperator,
  id,
  number,
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
    int line;
    int colmun;
    char take();
    bool isNum();
    bool isOp();
    std::vector<Token> tokenize();
  public: 
    std::string code;
    Tokenizer(std::string code);
};
