import codegen_def
import ../etc/enviroments
import noxen
import strformat

type
  NodeType* = enum
    Program,
    Num,
    ID,
    Str,
    Bool,
    varDeclare,
    varAssign,
    binaryExpr,
    Operator,
    Error

  Expr* = ref object 
    line*, colmun*:int
    codegen: proc(self: var Codegen): ValueType
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
    of Error:
      msg: string 
    of varDeclare:
      declare_name*: string
      declare_value*: Expr
    of varAssign:
      assign_name*: string
      assign_value*: Expr
    of binaryExpr: 
      left*: Expr 
      right*: Expr 
      operator*: Expr
    else:
      discard

proc MakeError*(msg: string, line, colmun: int): Expr=
  return Expr(kind: Error, msg: msg,line: line, colmun: colmun)

proc MakeProg*(body: seq[Expr], line: int, colmun: int): Expr =
  
  return Expr(kind: NodeType.Program, body: @[], line: line, colmun: colmun)


proc MakeID*(symbol: string, line: int, colmun: int): Expr =
  return Expr(kind:  NodeType.ID, symbol: symbol, line: line, colmun: colmun)



proc MakeOperator*(symbol: string, line: int, colmun: int): Expr =
  return Expr(kind:  NodeType.Operator, op: symbol, line: line, colmun: colmun)



proc MakeNum*(value: float, line: int, colmun: int):  Expr =
  return Expr(kind:  NodeType.Num, num_value: value, line: line, colmun: colmun)



proc MakeStr*(value: string, line: int, colmun: int): Expr =
  return Expr(kind:  NodeType.Str, str_value: value, line: line, colmun: colmun)



proc MakeBinaryExpr*(left: Expr, right: Expr, operator: Expr, line: int, colmun: int): Expr =
  var expr = Expr(kind:  NodeType.binaryExpr, left: left,right: right, operator: operator, line: line, colmun: colmun)
  expr.codegen = proc(self: var Codegen): ValueType =              
      var L = expr.left
      var R = expr.right
      var binop = expr.operator.op
  
      var left = L.codegen(self)
      var right = R.codegen(self)

      if left == null or right == null:
        return null
  
      #if not expr.isVaildBinaryExpr():
       # return self.TypeMissmatchE(expr, left, right)
      result = right
      var op: OP
      case binop        
        of "+":
          op = OP.OP_ADD
        of "-":
          op = OP.OP_SUB
        of "*":
          op = OP.OP_MUL
        of "/":
          op = OP.OP_DIV
      # MATH R0 R1
      self.body.emit(op, reg - 2, reg - 1)
      # optimization to prevent using too many regs we instead
      # store results of math into reg - 2 ex (8 + 8 + 8) ADD R0 R0 R1 then ADD R0 R0 R1
      reg -= 1
  return expr
proc MakeVarDeclartion*(name: string, value: Expr, line, colmun: int): Expr =
  return Expr(kind: varDeclare, declare_name: name, declare_value: value, line: line, colmun: colmun)


proc MakeVarAssignment*(name: string, value: Expr, line, colmun: int): Expr =
  return Expr(kind: varAssign, assign_name: name, assign_value: value, line: line, colmun: colmun)


proc error(self: Codegen, msg: string) =
  echo makeBox(msg & &"\nat line:{self.line}, colmun:{self.colmun}", "error", full_style=red)


proc `$$`*(self: Expr): string =
  case self.kind:
    of Num:
      return $self.num_value
    of Str:
      return $self.str_value
    of Operator:
      return $self.op
    of binaryExpr:
      return $$self.left & " " & $$self.operator & " " & $$self.right
    else:
      return ""

proc isVaildBinaryExpr*(expr: Expr): bool =
  var left = expr.left.kind
  var right = expr.right.kind
  var expr_left = expr.left
  var expr_right = expr.right

  while left == binaryExpr:
    if not expr.left.isVaildBinaryExpr() :  return false
    expr_left = expr_left.left
    left = expr_left.kind
  
  while right == binaryExpr: 
    if not expr.right.isVaildBinaryExpr(): return false 
    expr_right = expr_right.right
    right = expr_right.kind
  
  return (left == Str and (expr.operator.op == "-" or expr.operator.op == "+")) or
         (left == Num and right == Num)
 
proc TypeMissmatchE*(self: Codegen, expr: Expr, left: StaticType, right: StaticType): StaticType =
  self.error(&"""
type missmatch got 
left => {$$expr.left}:{$left}
right => {$$expr.right}:{$right} in expr {$$expr}""")
  return error



