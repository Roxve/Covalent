#include <iostream>
#include "AST.hpp"

using namespace std;

Expr::Expr(int line, int colmun) : line(line), colmun(colmun) {}

Program::Program(int line, int colmun) : Expr(line, colmun) {
  this->type = NodeType::program;
}

Num::Num(float value,int line, int colmun) : Expr(line, colmun) {
  this->type = NodeType::num;
  this->value = value;
}

BinaryExpr::BinaryExpr(Expr left_hand, Expr right_hand, string Operator, int line, int colmun) : Expr(line, colmun), left_hand(left_hand), right_hand(right_hand) {
  this->type = NodeType::binary_expr;
  this->Operator = Operator;
}
