# VM and codegen
import codegen_def
import AST
import parser_def
import tokenize
import parse_expr

var reg* = 0

proc generate(expr: Node): seq[byte] = 
  var res_bytes: seq[byte] = @[]
  case expr.node:
    of NodeType.binaryExpr:
      var binaryExpr = expr.BinaryExpr
      res_bytes.add(generate(binaryExpr.left))
      res_bytes.add(generate(binaryExpr.right))
      var op = binaryExpr.operator.OP.symbol
      case op
        of "+":
          res_bytes.add(byte(OP.OP_ADD))
          res_bytes.add(byte(reg))
          res_bytes.add(byte(reg - 2))
          res_bytes.add(byte(reg - 1))
  
      reg += 1 
  
    of NodeType.Num:
      var num = expr.num

      res_bytes.add(byte(OP.OP_LOAD))
      res_bytes.add(byte(reg))
      res_bytes.add(byte(num.value))
      reg += 1
    else:
      discard
  return res_bytes

proc productBytes*(this: var Parser): seq[byte] =
  var res_bytes: seq[byte] = @[]  
  while this.at().tok != TType.EOF:
    var expr = this.parse_expr()
    res_bytes.add(generate(expr))
  return res_bytes
  
