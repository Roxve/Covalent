#include <iostream>
#include <vector>
using namespace std;

enum NodeType {
  expr,
  program,
  num,
  binary_expr
};

class Expr {
  public:
    NodeType type;
    int line;
    int colmun;
    Expr(int line, int colmun);
};


class Program :public Expr {
  public:
    vector<Expr*> body;
    Program(int line, int colmun);
};

class Num :public Expr {
  public:
    float value;
    Num(float value, int line, int colmun);
};

class BinaryExpr :public Expr {
  public:
    Expr left_hand;
    Expr right_hand;
    string Operator;
    BinaryExpr(Expr left_hand, Expr right_hand, string Operator,int line, int colmun);
};
