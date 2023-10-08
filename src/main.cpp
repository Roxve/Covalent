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
  cout << "line: " << AST.body[0]->line;
  cout << "colmun: " << AST.body[0]->colmun;
  cout << "type: " << AST.body[0]->type;
  cout << "val: " << static_cast<Num*>(AST.body[0])->value;
  
  cout << endl;
  return 0;
}
