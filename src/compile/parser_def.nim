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


proc update*(this: var Parser) =
  this.line = this.tokenizer.line
  this.colmun = this.tokenizer.colmun

proc at*(this: var Parser): Token =
  this.update
  return this.tokenizer.current_token

proc take*(this: var Parser): Token =
  var prev = this.at
  this.last_token = prev
  discard this.tokenizer.next()
  return prev

proc error(this: var Parser, msg: string) = 
  echo makeBox(msg & &"\nat line:{this.line}, colmun:{this.colmun}", "error", full_style=red)

proc UnexceptedTokenE*(this: var Parser): Expr =
  var msg = &"unexcepted token '{this.at().value}' of type {this.at().tok}"
  this.error(msg) 
  return MakeError(msg, this.line, this.colmun)

proc UnexceptedTokenE*(this: var Parser, excepted: TType): Expr =
  var msg = &"unexcepted token '{this.at().value}' of type {this.at().tok}\nexcepted token of type '{excepted}'"
  this.error(msg) 
  return MakeError(msg, this.line, this.colmun)

proc excep*(this: var Parser, excepted: TType): (bool, Expr) =
  if this.take().tok != excepted:
    return (false, this.UnExceptedTokenE(excepted))
  return (true, Expr())

