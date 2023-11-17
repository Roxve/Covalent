import ../compile/codegen
import ../compile/codegen_def
import print

type
  Interpreter_results = enum
    success,
    error_division_by_zero
  COND = enum
    FL_POS,
    FL_NEG,
    Zero
  VM = object
    ip: int 
    reg: seq[uint32]
    R_COND: COND
    results: Interpreter_results
    results_eval: string 

func `<>`(a: seq[byte], b: seq[byte]): int =
  if a.len > b.len:
    return 1
  if a.len < b.len:
    return -1

  for i in 0..a.len:
    if a[i] > b[i]:
      return 1
    elif a[i] < b[i]:
      return -1
  return 0



proc signExtend(x: uint8): uint32 = 
    var res: uint32 = uint32(x)
    if (x shr (8 - 1) and 1) != 0:
        res = uint32(x or (0xFFFFFF shl 8))
    result = res

proc changeCond(vm: var VM, reg: int) = 
  if vm.reg[reg] == 0: 
    vm.R_COND = Zero 
  elif (vm.reg[reg] shr 31) == 1: 
    vm.R_COND = FL_NEG 
  else: 
    vm.R_COND = FL_POS

proc interpret*(bytecode: seq[byte]): VM =
  var vm = VM()
  vm.ip = 0
  while vm.ip < bytecode.len:
    var op = OP(bytecode[vm.ip])    
    
    vm.ip += 1
    case op: 
      of OP_LOAD: 
        var reg0 = bytecode[vm.ip]
        print reg0
        var imm = bytecode[vm.ip + 1] 
        print imm
        vm.ip += 2
        if vm.reg.len - 1 < int(reg0): 
          vm.reg.add(0)

        vm.reg[reg0] = signExtend(imm)
        print vm.reg
        vm.results = success  
        vm.results_eval = $imm
      of OP_ADD:
        var dist = int(bytecode[vm.ip])
        var reg1 = bytecode[vm.ip + 1] 
        var reg2 = bytecode[vm.ip + 2] 
  
        if vm.reg.len - 1 < dist:
          vm.reg.add(0)     
        vm.reg[dist] = vm.reg[reg1] + vm.reg[reg2]
    
        vm.changeCond(dist) 
        
        vm.ip += 3 
        vm.results = success 
        
        vm.results_eval = $vm.reg[dist]

      of OP_SUB:
        var dist = int(bytecode[vm.ip])
        var reg1 = bytecode[vm.ip + 1] 
        var reg2 = bytecode[vm.ip + 2] 
  
        if vm.reg.len - 1 < dist:
          vm.reg.add(0)     
        vm.reg[dist] = vm.reg[reg1] - vm.reg[reg2]
    
        vm.changeCond(dist) 
        
        vm.ip += 3 
        vm.results = success 
        
        vm.results_eval = $vm.reg[dist]     

      of OP_MUL:
        var dist = int(bytecode[vm.ip])
        var reg1 = bytecode[vm.ip + 1] 
        var reg2 = bytecode[vm.ip + 2] 
  
        if vm.reg.len - 1 < dist:
          vm.reg.add(0)     
        vm.reg[dist] = vm.reg[reg1] * vm.reg[reg2]
    
        vm.changeCond(dist) 
        
        vm.ip += 3 
        vm.results = success 
        
        vm.results_eval = $vm.reg[dist]     

      of OP_DIV:
        var dist = int(bytecode[vm.ip])
        var reg1 = bytecode[vm.ip + 1] 
        var reg2 = bytecode[vm.ip + 2] 
  
        if vm.reg.len - 1 < dist:
          vm.reg.add(0)     
        vm.reg[dist] = vm.reg[reg1] div vm.reg[reg2]
    
        vm.changeCond(dist) 
        
        vm.ip += 3 
        vm.results = success 
        
        vm.results_eval = $vm.reg[dist]    
      else:
        discard
  
  return vm
