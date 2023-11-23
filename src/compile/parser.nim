import AST
import parser_def
import parse_expr
import tokenize
import print

#proc mk_scope*(Type: ScopeType, parent: Option[Scope]): Scope =
 # return Scope(parent: parent, Type: Type)

proc make_parser*(src: string): Parser =
  var parser = Parser(line: 1, colmun: 0, tokenizer: make_tokenizer(src))
  parser.last_token = parser.tokenizer.next()
  #parser.current_scope = mk_scope(ScopeType.top, none(Scope))
  return parser

proc productAST*(self: var Parser): Expr =
  var body: seq[Expr] = @[]
  while self.at().tok != TType.EOF: 
    var expr = self.parse_expr()
    body.add(expr)
  return MakeProg(body, self.line, self.colmun)
