#include <iostream>
#include "CompileTime/parser.hpp"
#include <vector>
using namespace std;


int main() {
  cout << ">> ";
  string code;
  getline(cin, code);
  Parser parser(code);
  Program AST = parser.productAST();

  cout << endl;
  return 0;
}
