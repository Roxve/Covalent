import strformat
import tokenize
import options
import noxen

import ../etc/[enviroments, utils]
 

type
  OP*  = enum
    OP_CONSTS = byte(0) 
    OP_DEF
    TAG_INT
    TAG_FLOAT
    TAG_STR 
    OP_LOAD
    OP_LOAD_CONST
    OP_CALL # CALL DEF_ADDR
    OP_STRNAME
    OP_LOADNAME
    OP_ADD
    OP_SUB
    OP_MUL
    OP_DIV

  NodeType* = enum
    Program,
    Num,
    ID,
    Str,
    Bool,
    varDeclare,
    funcDeclare,
    methodDeclare,
    callExpr,
    memberExpr,
    varAssign,
    binaryExpr,
    Operator,
    Error

  ScopeType* = enum
    top,
    inside_func,
    inside_params,
    inside_call,
    variable_declaration_val,
    variable_assigment_val
type
  Codegen* = ref object
    line*, colmun*: int
    env*: Enviroment
    parser*: Parser
    consts_count*: int16
    const_objs*: seq[(RuntimeValue, int16)]
    const_bytes*: seq[byte]    
    body*: seq[byte]

  Expr* = ref object 
    line*, colmun*:int
    codegen*: proc(self : var Codegen): ValueType       
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
      msg*: string 
    of varDeclare:
      declare_name*: string
      declare_value*: Expr
      varKind*: Expr
    of varAssign:
      assign_name*: string
      assign_value*: Expr
    of funcDeclare:
      name*: string
      funcBody*: seq[Expr]    
      parameters*: seq[Expr]
    of callExpr:
      calle*: Expr
      args*: seq[Expr]
    of memberExpr:
      computed*: bool
      obj*: Expr
      member*: Expr
    of binaryExpr: 
      left*: Expr 
      right*: Expr 
      operator*: Expr
    else:
      discard

  Parser* = ref object
    line*, colmun*: int
    tokenizer*: Tokenizer
    last_token*: Token
    current_scope*: Scope
    parse_start*: proc(self: var Parser): Expr

  Scope* = ref object
    parent*: Option[Scope]
    Type*: ScopeType



var reg* = 0

proc emit*(bytes: var seq[byte],op: OP, reg0: int, reg1: int, reg2: int) =
  bytes.add(byte(op))
  bytes.add(byte(reg0))
  bytes.add(byte(reg1))
  bytes.add(byte(reg2))



proc emit*(bytes: var seq[byte],op: OP, reg0: int, byte0: byte, byte1: byte) =
  bytes.add(byte(op))
  bytes.add(byte(reg0))
  bytes.add(byte0)
  bytes.add(byte1)

proc emit*(bytes: var seq[byte],op: OP, reg0: int, bytesTo: seq[byte]) =
  bytes.add(byte(op))
  bytes.add(byte(reg0))
  bytes.add(bytesTo)
# emit str name and load name

proc emit*(bytes: var seq[byte],op: OP,  bytesTo: seq[byte],reg0: int) =
  bytes.add(byte(op))
  bytes.add(bytesTo)
  bytes.add(byte(reg0))

proc emit*(bytes: var seq[byte],op: OP, reg0: int, imm: int | float) =
  bytes.add(byte(op))
  bytes.add(byte(reg0))
  bytes.add(byte(imm))

proc emit*(bytes: var seq[byte],tag: OP, value: seq[byte]) =
  bytes.add(byte(tag))
  bytes.add(value)

proc emit*(bytes: var seq[byte],tag: OP, byteCount: int16,value: seq[byte]) =
  bytes.add(byte(tag))
  bytes.add(byteCount.to2Bytes)
  bytes.add(value)


proc addConst*(self: var Codegen, tag: OP,kind: ValueType ,bytes: seq[byte]): int16 =
  var aConsant = RuntimeValue(kind: kind,bytes: bytes)    
  for key, val in self.const_objs.items():
    if key == aConsant:
      return val
  
  self.const_bytes.emit(tag, bytes)
  inc self.consts_count
  self.const_objs.add((aConsant, self.consts_count))
  return self.consts_count

proc addConst*(self: var Codegen, tag: OP,kind: ValueType, byteCount: int16 ,bytes: seq[byte]): int16 =
  var aConsant = RuntimeValue(kind: kind,bytes: bytes)    
  for key, val in self.const_objs.items():
    if key == aConsant:
      return val
  
  self.const_bytes.emit(tag, byteCount ,bytes)
  inc self.consts_count 
  self.const_objs.add((aConsant, self.consts_count))
  return self.consts_count


  

proc `$$`*(self: Expr): string =
  case self.kind:
    of ID:
      return $self.symbol
    of Num:
      return $self.num_value
    of Str:
      return &"'{$self.str_value}'"
    of Operator:
      return $self.op
    of binaryExpr:
      return $$self.left & " " & $$self.operator & " " & $$self.right
    of varAssign:
      return &"{self.assign_name} := {$$self.assign_value}"
    else:
      return ""




proc MakeError*(msg: string, line, colmun: int): Expr=
  return Expr(kind: Error, msg: msg,line: line, colmun: colmun)

proc update*(self: var Parser) =
  self.line = self.tokenizer.line
  self.colmun = self.tokenizer.colmun

proc at*(self: var Parser): Token =
  self.update
  return self.tokenizer.current_token

proc take*(self: var Parser): Token =
  var prev = self.at
  self.last_token = prev
  discard self.tokenizer.next()
  return prev

proc error(self: var Parser, msg: string) = 
  echo makeBox(msg & &"\nat line:{self.line}, colmun:{self.colmun}", "error", full_style=red)

proc UnexceptedTokenE*(self: var Parser): Expr =
  var msg = &"unexcepted token '{self.at().value}' of type {self.at().tok}"
  self.error(msg) 
  return MakeError(msg, self.line, self.colmun)

proc UnexceptedTokenE*(self: var Parser, excepted: TType): Expr =
  var msg = &"unexcepted token '{self.at().value}' of type {self.at().tok}\nexcepted token of type '{excepted}'"
  self.error(msg) 
  return MakeError(msg, self.line, self.colmun)

proc excep*(self: var Parser, excepted: TType): (bool, Expr) =
  if self.at().tok != excepted:
    return (false, self.UnExceptedTokenE(excepted)) 
  discard self.take()
  return (true, Expr())

