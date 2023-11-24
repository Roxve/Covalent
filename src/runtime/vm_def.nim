import ../etc/utils
import ../etc/enviroments
  
type
  Interpreter_results* = enum
    success,
    error_division_by_zero,
    UNKNOWN_OP
  COND* = enum
    FL_POS,
    FL_NEG,
    FL_STR,
    Zero
  
  
  VM* = object
    ip*: int 
    reg*: seq[REG]
    consts*: seq[RuntimeValue]    
    R_COND*: COND
    results*: Interpreter_results
    results_eval*: string 
  REG* = object
    kind*: ValueType
    bytes*: seq[byte]



proc changeCond*(vm: var VM, reg: int) = 
  if vm.reg[reg].kind == ValueType.int:
    var val = makeInt(vm.reg[reg].bytes)
    if val == 0: 
      vm.R_COND = Zero 
    if (val shr 31) == 1: 
      vm.R_COND = FL_NEG 
    else: 
      vm.R_COND = FL_POS
    vm.results = success
    vm.results_eval = $vm.reg[reg]
  else:
    vm.R_COND = FL_STR
    vm.results = success
    vm.results_eval = vm.reg[reg].bytes.BytesToStr


proc checkRegs*(vm: var VM, num: byte | int)  =
  while vm.reg.len - 1 < int(num): 
    vm.reg.add(REG(bytes: @[]))
