import ../compile/codegen
import ../compile/codegen_def

type
  Interpreter_results = enum
    success,
    error_division_by_zero
  VM = object
    ip: int 
    reg: seq[byte]
    results: Interpreter_results
    results_eval: string

proc interpret*(bytecode: seq[byte]): VM =
  var vm = VM()
  vm.ip = 0
  while vm.ip < bytecode.len:
    var op = OP(bytecode[vm.ip]) 
    vm.ip += 1
    case op: 
      of OP_LOAD: 
        var reg0 = bytecode[vm.ip] 
        var imm = bytecode[vm.ip + 1] 
        vm.ip += 2  
        if vm.reg.len - 1 < int(reg0): 
          vm.reg.add(0)
        vm.reg[reg0] = imm 
        vm.results = success  
        vm.results_eval = $imm
      of OP_ADD:
        var reg0 = bytecode[vm.ip] 
        var reg1 = bytecode[vm.ip + 1] 
        var reg2 = bytecode[vm.ip + 2]
        
        if vm.reg.len - 1 < int(reg0): 
          vm.reg.add(0)     
        vm.reg[reg0] = vm.reg[reg1] + vm.reg[reg2]
        vm.ip += 3 
        vm.results = success 
        vm.results_eval = $vm.reg[reg0]
      else:
        discard
  
  return vm
