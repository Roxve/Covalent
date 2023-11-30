# VM and codegen

import compiler
import tokenize

import ../etc/[utils, enviroments]
import tables
import noxen
import strformat
import print
import math
import Options

proc isVaildBinaryExpr*(left: ValueType, right: ValueType, op: string): bool =
  return (left == str and (op == "-" or op == "+")) or
         ((left == ValueType.int or left == ValueType.float) and (right == ValueType.int or right == ValueType.float))
 
proc error(self: Codegen, msg: string) =
  echo makeBox(msg & &"\nat line:{self.line}, colmun:{self.colmun}", "error", full_style=red)

proc TypeMissmatchE*(self: Codegen, expr: Expr, left: ValueType, right: ValueType): ValueType =
  error(self,&"""
type missmatch got 
left => {$$expr.left}:{$left}
right => {$$expr.right}:{$right} in expr {$$expr}""")
  return error


proc UndeclaredIDE*(self: Codegen, expr: Expr): ValueType =
  self.error(&"undeclared id '{expr.symbol}'")
  return error

proc UndeclaredIDE*(self: Codegen, expr: string): ValueType =
  self.error(&"undeclared id '{expr}'")
  return error


proc DupDeclarationE(self: Codegen, name: string): ValueType = 
  self.error(&"id '{name}' already declared")
  return error

template NodeCodegen(code: untyped) =
  expr.codegen = proc(self {.inject.}: var Codegen): ValueType =
    code



proc MakeProg*(body: seq[Expr], line: int, colmun: int): Expr =
  
  return Expr(kind: NodeType.Program, body: @[], line: line, colmun: colmun)


proc MakeID*(symbol: string, line: int, colmun: int): Expr =
  var expr = Expr(kind:  NodeType.ID, symbol: symbol, line: line, colmun: colmun)
  NodeCodegen:
      var name = expr.symbol
      var (index, val) = self.env.getVarIndex(name)
      dprint: name

      if index == 0:
        return self.UndeclaredIDE(expr)
      result = val.kind
      self.body.emit(OP_LOADNAME, reg, int16(index).to2Bytes)
      reg += 1
  return expr

proc MakeOperator*(symbol: string, line: int, colmun: int): Expr =
  return Expr(kind:  NodeType.Operator, op: symbol, line: line, colmun: colmun)



proc MakeNum*(value: float, line: int, colmun: int):  Expr =
  var expr =  Expr(kind:  NodeType.Num, num_value: value, line: line, colmun: colmun)
  NodeCodegen:  
      var count = int16(0)
      if expr.num_value == round(expr.num_value):
        result = ValueType.int        
        count = self.addConst(TAG_INT, ValueType.int, uint32(expr.num_value).to4Bytes())
      else:
        result = ValueType.float
        count = self.addConst(TAG_FLOAT, ValueType.float, system.float32(expr.num_value).to4Bytes)
      # LOAD dist imm
      self.body.emit(OP_LOAD_CONST, reg, count.to2Bytes)
      reg += 1  
  return expr



proc MakeStr*(value: string, line: int, colmun: int): Expr =
  var expr = Expr(kind:  NodeType.Str, str_value: value, line: line, colmun: colmun)
  NodeCodegen:
      result = ValueType.str
      var count = self.addConst(TAG_STR, result,int16(expr.str_value.len), expr.str_value.StrToBytes)

      self.body.emit(OP_LOAD_CONST, reg, count.to2Bytes)
      reg += 1
  return expr

proc MakeFuncDeclaration*(name: string, parameters: seq[Expr], body: seq[Expr]): Expr =
  var expr = Expr(kind: NodeType.funcDeclare, name: name, parameters: parameters, funcBody: body)
  NodeCodegen:
    dprint: expr 
    return error
  return expr


proc MakeCallExpr*(calle: Expr, args: seq[Expr]): Expr =
  return Expr(kind: NodeType.callExpr, calle: calle, args: args)

proc MakeMemberExpr*(computed: bool, obj: Expr, member: Expr): Expr =
  return Expr(kind: NodeType.memberExpr, computed: computed, obj: obj, member: member)

proc MakeBinaryExpr*(left: Expr, right: Expr, operator: Expr, line: int, colmun: int): Expr =
  var expr = Expr(kind:  NodeType.binaryExpr, left: left,right: right, operator: operator, line: line, colmun: colmun)
  NodeCodegen:              
      var L = expr.left
      var R = expr.right
      var binop = expr.operator.op
  
      var left = L.codegen(self)
      var right = R.codegen(self)

      if left == error or right == error:
        return error
  
      if not isVaildBinaryExpr(left, right, expr.operator.op):
        return self.TypeMissmatchE(expr, left, right)
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
  var expr = Expr(kind: varDeclare, declare_name: name, declare_value: value, line: line, colmun: colmun)
  NodeCodegen:
      var name = expr.declare_name
      if self.env.resolve(name) != none(Enviroment):
        return self.DupDeclarationE(name)
      self.env.addVarIndex(name)
      var index = self.env.var_count      

      var val = expr.declare_value
      result = val.codegen(self)  
      self.env.setVar(index, RuntimeValue(kind: result))      
      # DIST_INDEX <= REG
      self.body.emit(OP_STRNAME, int16(index).to2Bytes(), reg - 1)
      reg -= 1
  return expr



proc MakeVarAssignment*(name: string, value: Expr, line, colmun: int): Expr =
  var expr = Expr(kind: varAssign, assign_name: name, assign_value: value, line: line, colmun: colmun)
  NodeCodegen:
      var name = expr.assign_name
      var (index, aval) = self.env.getVarIndex(name)
      if index == 0:
        return self.UndeclaredIDE(expr.assign_name)
      var val = expr.assign_value
      result = val.codegen(self)
      if aval.kind != result:
        return self.TypeMissmatchE(expr, aval.kind, result)
      # DIST_INDEX <= REG
      self.body.emit(OP_STRNAME, int16(index).to2Bytes(), reg - 1)
      reg -= 1
  return expr



proc productBytes*(this: var Parser): seq[byte] =
  var res_bytes: seq[byte] = @[]  
  
  var generator = Codegen(const_bytes: @[], body: @[], env: Enviroment(varibles: Table[uint16, RuntimeValue]()), parser: this)

  while this.at().tok != TType.EOF:
    var expr = this.parse_start(this)
  
    if expr.kind == Error:
      quit(1)
    var res = expr.codegen(generator)
    if res == ValueType.error:
      quit(1)

  var head = @[byte(OP_CONSTS)] & (generator.consts_count - 1).to2Bytes()
  res_bytes = head & generator.const_bytes & generator.body
  return res_bytes
  
