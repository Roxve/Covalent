import ../runtime/vm_def
import AST
import strformat
import noxen
import ../etc/utils

type
  OP*  = enum
    OP_CONSANTS = byte(0)
    TAG_INT
    TAG_FLOAT
    TAG_STR
    OP_LOAD_CONST
    OP_LOAD
    OP_ADD
    OP_SUB
    OP_MUL
    OP_DIV
  StaticType* = enum
    static_int
    static_str
    error
    dynamic
  Error = RootObj
  TypeMissmatch = object of Error
    left, right, expr: string
    
  Codegen* = object
    consants_count*: int16
    line*, colmun*: int
    consants*: seq[byte] 
    consant_objs*: seq[(consant, int16)]
    body*: seq[byte]



proc error(this: Codegen, msg: string) =
  echo makeBox(msg & &"\nat line:{this.line}, colmun:{this.colmun}", "error", full_style=red)


proc TypeMissmatchE*(this: Codegen, expr: Expr, left: StaticType, right: StaticType): StaticType =
  this.error(&"""
type missmatch got 
left => {$$expr.left}:{$left}
right => {$$expr.right}:{$right} in expr {$$expr}""")
  return error




var reg* = 0

proc emit*(bytes: var seq[byte],op: OP, reg0: int, reg1: int, reg2: int) =
  bytes.add(byte(op))
  bytes.add(byte(reg0))
  bytes.add(byte(reg1))
  bytes.add(byte(reg2))



proc emit*(bytes: var seq[byte],op: OP, reg0: int, byte0: byte, byte1: byte) =
  bytes.add(byte(op))
  bytes.add(byte(reg0))
  bytes.add(byte0)
  bytes.add(byte1)

proc emit*(bytes: var seq[byte],op: OP, reg0: int, bytesTo: seq[byte]) =
  bytes.add(byte(op))
  bytes.add(byte(reg0))
  bytes.add(bytesTo)

proc emit*(bytes: var seq[byte],op: OP, reg0: int, imm: int | float) =
  bytes.add(byte(op))
  bytes.add(byte(reg0))
  bytes.add(byte(imm))

proc emit*(bytes: var seq[byte],tag: OP, value: seq[byte]) =
  bytes.add(byte(tag))
  bytes.add(value)

proc emit*(bytes: var seq[byte],tag: OP, byteCount: int16,value: seq[byte]) =
  bytes.add(byte(tag))
  bytes.add(byteCount.to2Bytes)
  bytes.add(value)


proc addConst*(this: var Codegen, tag: OP,ctype: const_type ,bytes: seq[byte]): int16 =
  var aConsant = consant(ctype: ctype,bytes: bytes)    
  for key, val in this.consant_objs.items():
    if key == aConsant:
      return val
  
  this.consants.emit(tag, bytes)
  inc this.consants_count 
  this.consant_objs.add((aConsant, this.consants_count))
  return this.consants_count

proc addConst*(this: var Codegen, tag: OP,ctype: const_type, byteCount: int16 ,bytes: seq[byte]): int16 =
  var aConsant = consant(ctype: ctype,bytes: bytes)    
  for key, val in this.consant_objs.items():
    if key == aConsant:
      return val
  
  this.consants.emit(tag, byteCount ,bytes)
  inc this.consants_count 
  this.consant_objs.add((aConsant, this.consants_count))
  return this.consants_count
