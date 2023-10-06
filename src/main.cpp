#include <iostream>
#include "CompileTime/Tokenizer.hpp"
#include <vector>
using namespace std;


int main() {
  cout << ">> ";
  string code;
  getline(cin, code);
  Tokenizer tokenizer = Tokenizer(code);
  vector<Token> tokens = tokenizer.tokenize();

  cout << endl;
  return 0;
}
