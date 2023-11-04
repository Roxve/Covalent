type
  NodeType* = enum
    Num,
    ID,
    Str,
    Bool,
    binaryExpr,
    Operator,

  Expr* = object of RootObj
    node*: NodeType
    line*, colmun*: int
  IDVal* = ref object of Expr
    symbol*: string
  OperatorVal* = ref object of Expr
    symbol*: string

  NumVal* = ref object of Expr
    value*: int | float
  StrVal* = ref object of Expr
    value: string

  BinaryExpr* = ref object of Expr
    left: Expr
    right: Expr
    operator: OperatorVal

proc Make_ID*(symbol: string, line: int, colmun: int): IDVal =
  return IDVal(node: NodeType.ID, symbol: symbol, line: line, colmun: colmun)

proc Make_Operator*(symbol: string, line: int, colmun: int): OperatorVal =
  return OperatorVal(node: NodeType.ID, symbol: symbol, line: line, colmun: colmun)

proc Make_Num*(value: int | float, line: int, colmun: int): NumVal =
  return NumVal(node: NodeType.ID, value: value, line: line, colmun: colmun)

proc Make_Str*(value: string, line: int, colmun: int): StrVal =
  return StrVal(node: NodeType.ID, value: value, line: line, colmun: colmun)

proc Make_BinaryExpr*(left: Expr, right: Expr, operator: OperatorVal, line: int, colmun: int): BinaryExpr =
  return BinaryExpr(node: NodeType.ID, left: left,right: right, operator: operator, line: line, colmun: colmun)
