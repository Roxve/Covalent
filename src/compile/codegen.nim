# VM and codegen
import codegen_def
import AST
import parser_def
import tokenize
import parse_expr
import math
import ../runtime/vm_def


proc generate(this: var Codegen, expr: Node): StaticType = 
  var bytes: seq[byte] = @[]
  var constant_bytes: seq[byte] = @[]
  var btype: StaticType = dynamic

  case expr.node:
    of NodeType.binaryExpr:
      var binaryExpr = expr.BinaryExpr
      var left_node = binaryExpr.left.node
      var right_node = binaryExpr.right.node
      var binop = binaryExpr.operator.OP.symbol
  
      var left = this.generate(binaryExpr.left)
      var right = this.generate(binaryExpr.right)

      if left == error or right == error:
        return error
  
      if left != right and (left != static_str or right != static_str):
        return this.TypeMissmatchE($left_node, $right_node, $left_node & binop & $right_node)
      btype = right
      var op: OP
      case binop        
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
      var count = this.consants_count
      if num.value == round(num.value):
        btype = static_int
        var number = consant(ctype: cint,bytes:uint32(num.value).to4Bytes())    
        var found = false
        for key, val in this.consant_objs.items():
          if key == number:
            count = val
            found = true
            break; 
        if not found:
          constant_bytes.emit(TAG_INT, number.bytes)
          btype = static_int
          inc this.consants_count 
          count = this.consants_count
          this.consant_objs.add((number, count))
      # LOAD dist imm
      bytes.emit(OP_LOAD_CONST, reg, byte((count shr 8) and 0xFF), byte(count and 0xFF))
      reg += 1
    else:
      discard
  this.consants.add(constant_bytes)
  this.body.add(bytes)
  return btype

proc productBytes*(this: var Parser): seq[byte] =
  var res_bytes: seq[byte] = @[]  
  var generator = Codegen(consants: @[], body: @[])
  while this.at().tok != TType.EOF:
    var expr = this.parse_expr()

    var res = generator.generate(expr)
    if res == error:
      quit(1)

  var head = @[byte(OP_CONSANTS)] & (generator.consants_count - 1).to2Bytes()
  res_bytes = head & generator.consants & generator.body
  return res_bytes
  
