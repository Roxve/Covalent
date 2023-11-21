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
    else: 
      var e = this.UnexceptedTokenE()
      discard this.take()
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


proc parse_expr*(this: var Parser): Expr = 
  return (this.parse_additive_expr())