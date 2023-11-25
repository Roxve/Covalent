import ../etc/[enviroments, utils]

type
  OP*  = enum
    OP_CONSTS = byte(0)
    TAG_INT
    TAG_FLOAT
    TAG_STR
    OP_LOAD_CONST
    OP_LOAD
    OP_STRNAME
    OP_LOADNAME
    OP_ADD
    OP_SUB
    OP_MUL
    OP_DIV
  StaticType* = enum
    static_int
    static_str
    error
    dynamic
  Codegen* = object
    consts_count*: int16
    line*, colmun*: int
    const_bytes*: seq[byte] 
    env*: Enviroment
    const_objs*: seq[(RuntimeValue, int16)]
    body*: seq[byte]




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
# emit str name and load name

proc emit*(bytes: var seq[byte],op: OP,  bytesTo: seq[byte],reg0: int) =
  bytes.add(byte(op))
  bytes.add(bytesTo)
  bytes.add(byte(reg0))

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


proc addConst*(self: var Codegen, tag: OP,kind: ValueType ,bytes: seq[byte]): int16 =
  var aConsant = RuntimeValue(kind: kind,bytes: bytes)    
  for key, val in self.const_objs.items():
    if key == aConsant:
      return val
  
  self.const_bytes.emit(tag, bytes)
  inc self.consts_count
  self.const_objs.add((aConsant, self.consts_count))
  return self.consts_count

proc addConst*(self: var Codegen, tag: OP,kind: ValueType, byteCount: int16 ,bytes: seq[byte]): int16 =
  var aConsant = RuntimeValue(kind: kind,bytes: bytes)    
  for key, val in self.const_objs.items():
    if key == aConsant:
      return val
  
  self.const_bytes.emit(tag, byteCount ,bytes)
  inc self.consts_count 
  self.const_objs.add((aConsant, self.consts_count))
  return self.consts_count
