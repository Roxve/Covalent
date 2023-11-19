type
  Interpreter_results* = enum
    success,
    error_division_by_zero,
    UNKNOWN_OP
  COND* = enum
    FL_POS,
    FL_NEG,
    Zero
  const_type* = enum
    cint
  
  consant* = object
    ctype*: const_type
    bytes*: seq[byte]
  
  VM* = object
    ip*: int 
    reg*: seq[REG]
    consants*: seq[consant]    
    R_COND*: COND
    results*: Interpreter_results
    results_eval*: string 
  REG* = object
    vtype*: const_type
    bytes*: seq[byte]

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
  else:
    return
         signExtend(x[0] shl 24) or
         signExtend(x[1] shl 16) or
         signExtend(x[2] shl 8) or
         signExtend(x[3])

proc changeCond*(vm: var VM, reg: int) = 
  var val = makeInt(vm.reg[reg].bytes)
  if val == 0: 
    vm.R_COND = Zero 
  elif (val shr 31) == 1: 
    vm.R_COND = FL_NEG 
  else: 
    vm.R_COND = FL_POS
  vm.results = success
  vm.results_eval = $vm.reg[reg]


proc checkRegs*(vm: var VM, num: byte | int)  =
  if vm.reg.len - 1 < int(num): 
    vm.reg.add(REG())
