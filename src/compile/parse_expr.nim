import parser_def
import tokenize
import AST
import strutils

proc parse_primary_expr(this: var Parser): Expr =
  case this.at().tok:
    of TType.num:
      return MakeNum(parseFloat(this.take().value), this.line, this.colmun)
    of TType.str:
        return MakeStr(this.take.value, this.line, this.colmun)
    of TType.id:
        return MakeID(this.take.value, this.line, this.colmun)
    of TType.openParen:
      discard this.take
      var expr = this.parse_start(this)
      discard this.excep(TType.closeParen)
      return expr
    else: 
      var e = this.UnexceptedTokenE()
      discard this.take()
      quit(1)
      return e


proc parse_multipictive_expr(this: var Parser): Expr =
  var left = this.parse_primary_expr()
  while this.at().value == "*" or this.at().value == "/" or this.at().value == "%":
    let operator = MakeOperator(this.take().value, this.line, this.colmun)
    var right = this.parse_primary_expr()
    left = MakeBinaryExpr(left, right, operator, this.line, this.colmun)
  return left



proc parse_additive_expr(this: var Parser): Expr =
  var left = this.parse_multipictive_expr()
  while this.at().value == "+" or this.at().value == "-":
    let operator = MakeOperator(this.take().value, this.line, this.colmun)
    var right = this.parse_multipictive_expr()
    left = MakeBinaryExpr(left, right, operator, this.line, this.colmun)
  return left

proc parse_var_assign(this: var Parser): Expr =
  var left = this.parse_additive_expr
  if this.at().tok == assign:
    if left.kind != ID:
      return this.UnexceptedTokenE()
    discard this.take
    var right = this.parse_additive_expr
    left = MakeVarAssignment(left.symbol, right, this.line, this.colmun)  
  return left

proc parse_var_declaration(this: var Parser): Expr =
  if this.at().tok == set_kw:
    discard this.take()
    var name = this.take()
    if name.tok != id:
      return this.UnexceptedTokenE()
  
    let (found, exception) = this.excep(to_kw)
    if not found:
      return exception
    
    var value = this.parse_var_assign()
    return MakeVarDeclartion(name.value, value, this.line, this.colmun)
  
  return this.parse_var_assign()

proc parse_expr*(this: var Parser): Expr = 
  this.parse_start = proc(self: var Parser): Expr = return self.parse_var_declaration()
  return (this.parse_var_declaration)
