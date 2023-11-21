# VM and codegen
import codegen_def
import AST
import parser_def
import tokenize
import parse_expr
import math
import ../runtime/vm_def
import ../etc/utils

proc generate(this: var Codegen, expr: Expr): StaticType = 
  var bytes: seq[byte] = @[]
  var constant_bytes: seq[byte] = @[]
  var btype: StaticType = dynamic

  case expr.kind:
    of NodeType.binaryExpr:
      var left_node = expr.left
      var right_node = expr.right
      var binop = expr.operator.op
  
      var left = this.generate(left_node)
      var right = this.generate(right_node)

      if left == error or right == error:
        return error
  
      if not expr.isVaildBinaryExpr():
        return this.TypeMissmatchE(expr, left, right)
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
      var count = this.consants_count
      if expr.num_value == round(expr.num_value):
        btype = static_int
        
        count = this.addConst(TAG_INT, const_type.cint, uint32(expr.num_value).to4Bytes())
      # LOAD dist imm
      bytes.emit(OP_LOAD_CONST, reg, count.to2Bytes)
      reg += 1
    of NodeType.Str:
      btype = static_str
      var count = this.addConst(TAG_STR, const_type.cstr,int16(expr.str_value.len), expr.str_value.StrToBytes)

      bytes.emit(OP_LOAD_CONST, reg, count.to2Bytes)
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
    if expr.kind == Error:
      quit(1)
    var res = generator.generate(expr)
    if res == error:
      quit(1)

  var head = @[byte(OP_CONSANTS)] & (generator.consants_count - 1).to2Bytes()
  res_bytes = head & generator.consants & generator.body
  return res_bytes
  
