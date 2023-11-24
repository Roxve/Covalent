# VM and codegen
import codegen_def
import AST
import parser_def
import tokenize
import parse_expr
import math
import ../runtime/vm_def
import ../etc/utils
import ../etc/enviroments
import tables
import print
import Options


proc productBytes*(this: var Parser): seq[byte] =
  var res_bytes: seq[byte] = @[]  
  var generator = Codegen(const_bytes: @[], body: @[], env: Enviroment(varibles: Table[uint16, RuntimeValue]()))

  while this.at().tok != TType.EOF:
    var expr = this.parse_expr()
    if expr.kind == Error:
      quit(1)
    var res = expr.codegen(generator)
    if res == ValueType.error:
      quit(1)

  var head = @[byte(OP_CONSTS)] & (generator.consts_count - 1).to2Bytes()
  res_bytes = head & generator.const_bytes & generator.body
  return res_bytes
  
