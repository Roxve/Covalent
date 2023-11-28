import parser_def
import tokenize
import AST
import strutils
import print
proc parse_primary_expr(self: var Parser): Expr =
  case self.at().tok:
    of TType.num:
      return MakeNum(parseFloat(self.take().value), self.line, self.colmun)
    of TType.str:
        return MakeStr(self.take.value, self.line, self.colmun)
    of TType.id:
        return MakeID(self.take.value, self.line, self.colmun)
    of TType.openParen:
      discard self.take
      var expr = self.parse_start(self)
      discard self.excep(TType.closeParen)
      return expr
    else: 
      var e = self.UnexceptedTokenE()
      discard self.take()
      quit(1)



proc parse_multipictive_expr(self: var Parser): Expr =
  var left = self.parse_primary_expr()
  while self.at().value == "*" or self.at().value == "/" or self.at().value == "%":
    let operator = MakeOperator(self.take().value, self.line, self.colmun)
    var right = self.parse_primary_expr()
    left = MakeBinaryExpr(left, right, operator, self.line, self.colmun)
  return left



proc parse_additive_expr(self: var Parser): Expr =
  var left = self.parse_multipictive_expr()
  while self.at().value == "+" or self.at().value == "-":
    let operator = MakeOperator(self.take().value, self.line, self.colmun)
    var right = self.parse_multipictive_expr()
    left = MakeBinaryExpr(left, right, operator, self.line, self.colmun)
  return left

proc parse_var_assign(self: var Parser): Expr =
  var left = self.parse_additive_expr
  if self.at().tok == assign:
    if left.kind != ID:
      return self.UnexceptedTokenE()
    discard self.take
    var right = self.parse_additive_expr
    left = MakeVarAssignment(left.symbol, right, self.line, self.colmun)  
  return left

# used for lists and args
proc parse_list_args(self: var Parser): seq[Expr] =
  var items: seq[Expr] = @[]
  var item = self.parse_start(self)
  items.add(item)
  while self.at().tok == comma: 
    discard self.take()
    items.add(self.parse_start(self)) 
  return items

proc parse_args(self: var Parser): seq[Expr] =
  result = @[]
  discard self.take
  if self.at.value != ")":
    result = self.parse_list_args
    let (found, exception) = self.excep(closeParen)
    if not found:
      return @[exception]


proc parse_func_declaration(self: var Parser, name: string, ident: int): Expr = 


  var argsl = self.parse_args
    
  if argsl.len > 0 and argsl[0].kind == Error: return argsl[0]    
  for arg in argsl: 
    if arg.kind != ID: 
      return self.UnexceptedTokenE
  let (found, exception) = self.excep(to_kw)
  if not found:
    return exception
  var body: seq[Expr] = @[] 
  print ident 
  print self.at().colmun
  while self.at().colmun > ident: 
    body.add(self.parse_start(self))
  return MakeFuncDeclaration(name=name,argsl, body)
 
  
  
proc parse_var_declaration(self: var Parser): Expr =
  if self.at().tok == set_kw: 
    var ident = self.take().colmun - 3
    var name = self.take()
    if name.tok != id:
      return self.UnexceptedTokenE()

    if self.at().value == "(": return self.parse_func_declaration(name.value,ident)
    
    let (found, exception) = self.excep(to_kw)
    if not found:
      return exception
    
    var value = self.parse_var_assign()
    return MakeVarDeclartion(name.value, value, self.line, self.colmun)
  
  return self.parse_var_assign()

proc parse_expr*(self: var Parser): Expr = 
  self.parse_start = proc(self: var Parser): Expr = return self.parse_var_declaration()
  return (self.parse_var_declaration)
