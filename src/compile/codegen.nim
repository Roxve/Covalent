# VM and codegen
import codegen_def
import AST
import parser_def
import tokenize
import parse_expr




var reg* = 0

proc emit(bytes: var seq[byte],op: OP, reg0: int, reg1: int, reg2: int) =
  bytes.add(byte(op))
  bytes.add(byte(reg0))
  bytes.add(byte(reg1))
  bytes.add(byte(reg2))


proc emit(bytes: var seq[byte],op: OP, reg0: int, imm: int | float) =
  bytes.add(byte(op))
  bytes.add(byte(reg0))
  bytes.add(byte(imm))

proc generate(expr: Node): seq[byte] = 
  var bytes: seq[byte] = @[]
  case expr.node:
    of NodeType.binaryExpr:
      var binaryExpr = expr.BinaryExpr
      bytes.add(generate(binaryExpr.left))
      bytes.add(generate(binaryExpr.right))
      var op: OP
      case binaryExpr.operator.OP.symbol        
        of "+":
          op = OP.OP_ADD
        of "-":
          op = OP.OP_SUB
        of "*":
          op = OP.OP_MUL
        of "/":
          op = OP.OP_DIV
      # MATH R0 R0 R1
      bytes.emit(op, reg - 2, reg - 2, reg - 1)
      # optimization to prevent using too many regs we instead
      # store results of math into reg - 2 ex (8 + 8 + 8) ADD R0 R0 R1 then ADD R0 R0 R1
      reg -= 1
 
    of NodeType.Num:
      var num = expr.num
      # LOAD dist imm
      bytes.emit(OP_LOAD, reg, num.value)
      reg += 1
    else:
      discard
  return bytes

proc productBytes*(this: var Parser): seq[byte] =
  var res_bytes: seq[byte] = @[]  
  while this.at().tok != TType.EOF:
    var expr = this.parse_expr()
    res_bytes.add(generate(expr))
  return res_bytes
  
