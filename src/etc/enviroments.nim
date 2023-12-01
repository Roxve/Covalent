import tables
import Options


type

  ValueType* = enum
    int,
    float,
    str,
    null,
    error

  RuntimeValue* = object
    kind*: ValueType
    bytes*: seq[byte]
  
  Enviroment* = ref object
    parent*: Option[Enviroment]
    varibles*: Table[uint16, RuntimeValue]
    varible_names*: Table[string, uint16]
    var_count*: uint16 = 0


proc MakeEnv*(parent: Option[Enviroment]): Enviroment = 
  return Enviroment(parent: parent)

proc resolve*(this: Enviroment, index: uint16): Option[Enviroment] =
  
  if this.varibles.contains(index): return some(this)
  if isSome(this.parent):
    return this.parent.get.resolve(index)
  return none(Enviroment)


proc resolve*(this: Enviroment, index: string): Option[Enviroment] =
  
  if this.varible_names.contains(index): return some(this)
  if isSome(this.parent):
    return this.parent.get.resolve(index)
  return none(Enviroment)
  
 
proc setVar*(this: Enviroment,index: uint16, value: RuntimeValue) = this.varibles[index] = value

proc addVarIndex*(this: Enviroment, name: string) =
  inc this.var_count
  this.varible_names[name] = this.var_count

proc getVarVal*(this: Enviroment, index: uint16): RuntimeValue = 
  var env = this.resolve(index)
  if env.isNone():
    return RuntimeValue(kind: null)
  return env.get.varibles[index]


proc getVarIndex*(this: Enviroment, index: string): (uint16, RuntimeValue) =
  var env = this.resolve(index)
  if env.isNone():
    return (0, RuntimeValue(kind: null))
  var val = env.get.varible_names[index] 
  echo env.get.varibles 
  var runtime = RuntimeValue(kind: null)
  try:
    runtime = env.get.varibles[val]
  except: 
    runtime = RuntimeValue(kind: null)
  return (val, runtime)
