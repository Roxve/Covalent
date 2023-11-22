import ../compile/AST
import strutils
import unicode
import sequtils
import encodings
import sugar
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


proc signExtend*(x: uint8): uint32 = 
    var res: uint32 = uint32(x)
    if (x shr (8 - 1) and 1) != 0:
        res = uint32(x or (0xFFFFFF shl 8))
    result = res

proc makeInt*(x: seq[byte]): uint32 =
  if x.len == 2:
    return
           signExtend(x[0] shl 8) or
           signExtend(x[1])
  elif x.len == 1:
    return signExtend(x[0])
  else:
    return
         signExtend(x[0] shl 24) or
         signExtend(x[1] shl 16) or
         signExtend(x[2] shl 8) or
         signExtend(x[3])

proc `$$`*(this: Expr): string =
  case this.kind:
    of Num:
      return $this.num_value
    of Str:
      return $this.str_value
    of Operator:
      return $this.op
    of binaryExpr:
      return $$this.left & " " & $$this.operator & " " & $$this.right
    else:
      return ""

proc StrToBytes*(str: string): seq[byte] =
  return cast[seq[byte]](str)
proc BytesToStr*(bytes: var seq[byte]): string = 
  bytes = bytes.filter(proc(b: byte): bool = b != 0)  
  var str = cast[string](bytes)  
  return str  
