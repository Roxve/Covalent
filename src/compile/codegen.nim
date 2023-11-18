# VM and codegen
import codegen_def
import AST
import parser_def
import tokenize
import parse_expr
import math



proc generate(this: var Codegen, expr: Node): seq[byte] = 
  var bytes: seq[byte] = @[]
  var constant_bytes: seq[byte] = @[]
  case expr.node:
    of NodeType.binaryExpr:
      var binaryExpr = expr.BinaryExpr
      discard this.generate(binaryExpr.left)
      discard this.generate(binaryExpr.right)
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
      # MATH R0 R1
      bytes.emit(op, reg - 2, reg - 1)
      # optimization to prevent using too many regs we instead
      # store results of math into reg - 2 ex (8 + 8 + 8) ADD R0 R0 R1 then ADD R0 R0 R1
      reg -= 1
 
    of NodeType.Num:
      var num = expr.num
      if num.value == round(num.value):
        var number = uint32(num.value)
        # TODO IMPLENTE A FUNCTION TO DO THI    
        constant_bytes.emit(TAG_INT, number.to4Bytes())
      inc this.consants_count 
      # LOAD dist imm
      bytes.emit(OP_LOAD_CONST, reg, byte((this.consants_count shr 8) and 0xFF), byte(this.consants_count and 0xFF))
      reg += 1
    else:
      discard
  this.consants.add(constant_bytes)
  this.body.add(bytes)
  return constant_bytes & bytes

proc productBytes*(this: var Parser): seq[byte] =
  var res_bytes: seq[byte] = @[]  
  var generator = Codegen(consants: @[], body: @[])
  while this.at().tok != TType.EOF:
    var expr = this.parse_expr()
    discard generator.generate(expr)

  var head = @[byte(OP_CONSANTS)] & (generator.consants_count - 1).to2Bytes()
  res_bytes = head & generator.consants & generator.body
  return res_bytes
  
