import ../etc/utils

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
    cstr
  
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
