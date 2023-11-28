import tokenize
import AST
import options
import noxen
import strformat
type
  ScopeType* = enum
    top,
    inside_func,
    inside_params,
    inside_call,
    variable_declaration_val,
    variable_assigment_val
  Scope* = ref object
    parent*: Option[Scope]
    Type*: ScopeType
  Parser* = object
    line*, colmun*: int
    tokenizer*: Tokenizer
    last_token*: Token
    current_scope*: Scope
    parse_start*: proc(self: var Parser): Expr


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

