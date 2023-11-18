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
  Codegen* = object
    consants_count*: int16
    consants*: seq[byte]
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

proc emit*(bytes: var seq[byte],op: OP, reg0: int, imm: int | float) =
  bytes.add(byte(op))
  bytes.add(byte(reg0))
  bytes.add(byte(imm))

proc emit*(bytes: var seq[byte],tag: OP, value: seq[byte]) =
  bytes.add(byte(tag))
  bytes.add(value)
  


proc to4Bytes*(val: int | uint32 | int32): seq[byte] =
    var bytes: seq[byte] = @[]
    bytes.add(byte((val shr 24) and 0xFF))
    bytes.add(byte((val shr 16) and 0xFF))
    bytes.add(byte((val shr 8) and 0xFF))
    bytes.add(byte(val and 0xFF))
    return bytes

proc to2Bytes*(val: int16): seq[byte] =
    var bytes: seq[byte] = @[]
    bytes.add(byte((val shr 8) and 0xFF))
    bytes.add(byte(val and 0xFF))
    return bytes
