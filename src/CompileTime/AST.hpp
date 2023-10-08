#include <iostream>>
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


class Program : Expr {
  public:
    Expr* body;
    Program(int line, int colmun);
};

class Num : Expr {
  public:
    float value;
    Num(float value, int line, int colmun);
};

class BinaryExpr : Expr {
  public:
    Expr left_hand;
    Expr right_hand;
    string Operator;
    BinaryExpr(Expr left_hand, Expr right_hand, string Operator,int line, int colmun);
};
