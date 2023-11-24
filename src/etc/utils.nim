import unicode
import sequtils
type intBytes = array[0..3, byte]
type intVal = uint32
type int16Bytes = array[0..1, byte]

proc to4Bytes*(input: int | uint32 | int32): seq[byte] =
    var bytes: seq[byte] = toSeq(cast[intBytes](input))
    return bytes

proc seqToIntBytes*(val: seq[byte]): intBytes =
  result = [byte(0), 0, 0,0]
  if val.len > 0:
    for i in 0..(val.len - 1):
      result[i] = val[i]
  echo result
proc to2Bytes*(val: int16): seq[byte] =
    var bytes: seq[byte] = toSeq(cast[int16Bytes](val))
    return bytes
proc signExtend*(x: uint8): uint32 = 
    var res: uint32 = uint32(x)
    if (x shr (8 - 1) and 1) != 0:
        res = uint32(int(x) or (0xFFFFFF shl 8))
    result = res

proc makeInt*(x: seq[byte]): uint32 =
  echo x
  result = uint32(cast[intVal](x.seqToIntBytes))



proc StrToBytes*(str: string): seq[byte] =
  return cast[seq[byte]](str)
proc BytesToStr*(bytes: var seq[byte]): string = 
  bytes = bytes.filter(proc(b: byte): bool = b != 0)  
  var str = cast[string](bytes)  
  return str  
