type
  NodeType* = enum
    Program,
    Num,
    ID,
    Str,
    Bool,
    binaryExpr,
    Operator,

  Expr* = ref object 
    line*, colmun*:int
    case kind*: NodeType 
    of Program: 
      body*: seq[Expr] 
    of ID: 
      symbol*: string
    of Operator: 
      op*: string
    of Num: 
      num_value*: float
    of Str: 
      str_value*: string
    of binaryExpr: 
      left*: Expr 
      right*: Expr 
      operator*: Expr
    else:
      discard

proc Make_Prog*(body: seq[Expr], line: int, colmun: int): Expr =
  
  return Expr(kind: NodeType.Program, body: @[], line: line, colmun: colmun)


proc Make_ID*(symbol: string, line: int, colmun: int): Expr =
  return Expr(kind:  NodeType.ID, symbol: symbol, line: line, colmun: colmun)



proc Make_Operator*(symbol: string, line: int, colmun: int): Expr =
  return Expr(kind:  NodeType.Operator, op: symbol, line: line, colmun: colmun)



proc Make_Num*(value: float, line: int, colmun: int):  Expr =
  return Expr(kind:  NodeType.Num, num_value: value, line: line, colmun: colmun)



proc Make_Str*(value: string, line: int, colmun: int): Expr =
  return Expr(kind:  NodeType.Str, str_value: value, line: line, colmun: colmun)



proc Make_BinaryExpr*(left: Expr, right: Expr, operator: Expr, line: int, colmun: int): Expr =
  return Expr(kind:  NodeType.binaryExpr, left: left,right: right, operator: operator, line: line, colmun: colmun)
