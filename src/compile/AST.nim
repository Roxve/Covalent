type
  NodeType* = enum
    Program,
    Num,
    ID,
    Str,
    Bool,
    binaryExpr,
    Operator,
    
  Expr* = object of RootObj
    line*, colmun*: int
  Prog* = object of Expr
    body*: seq[Node]

  IDVal* = object of Expr
    symbol*: string
  OperatorVal* = object of Expr
    symbol*: string

  NumVal* = object of Expr
    value*: float
  StrVal* = object of Expr
    value: string

  BinaryExpr* = object of Expr
    left: Node
    right: Node
    operator: Node
  Node* = ref object
    case node*: NodeType
    of Program: prog*: Prog
    of ID: ID*: IDVal
    of Operator: OP*: OperatorVal
    of Num: num*: NumVal
    of Str: str*: StrVal
    of binaryExpr: BinaryExpr*: BinaryExpr
    else:
      discard

proc Make_Prog*(body: seq[Node], line: int, colmun: int): Node =
  
  return Node(node: NodeType.Program, prog: Prog( line: line, colmun: colmun, body: body))


proc Make_ID*(symbol: string, line: int, colmun: int): Node =
  return Node(node: NodeType.ID, ID:IDVal(symbol: symbol, line: line, colmun: colmun))



proc Make_Operator*(symbol: string, line: int, colmun: int): Node =
  return Node(node: NodeType.Operator, OP: OperatorVal(symbol: symbol, line: line, colmun: colmun))



proc Make_Num*(value: float, line: int, colmun: int):  Node =
  return Node(node: NodeType.Num, num: NumVal(value: value, line: line, colmun: colmun))



proc Make_Str*(value: string, line: int, colmun: int): Node =
  return Node(node: NodeType.Str, str: StrVal(value: value, line: line, colmun: colmun))



proc Make_BinaryExpr*(left: Node, right: Node, operator: Node, line: int, colmun: int): Node =
  return Node(node: NodeType.binaryExpr, BinaryExpr: BinaryExpr(left: left,right: right, operator: operator, line: line, colmun: colmun))
