import ../compile/codegen
import ../compile/codegen_def

type
  Interpreter_results = enum
    success,
    error_division_by_zero
  VM = object
    ip: int 
    reg: seq[seq[byte]]
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


template defineBinAdd() =
  func `+++`(a: var seq[byte], b: var seq[byte]): seq[byte] =
    var res: seq[byte] = @[]
  
    # negative
    if a.len != 0 and b.len != 0 and a[0] == 45 and b[0] == 45:
      a.del(0) 
      b.del(0)
      res = @[byte(45)] & a --- b
    # a is negative
    elif a.len != 0 and a[0] == 45:
      a.del(0)
      if a <> b == -1: 
        res = a --- b
      elif a <> b == 0:
        res = @[byte(0)]
      else:
        res = @[byte(45)] & a --- b
  
    elif b.len != 0 and b[0] == 45:
      a.del(0)
      if a <> b == -1: 
        res = a --- b
      elif a <> b == 0:
        res = @[byte(0)]
      else:
        res = @[byte(45)] & a --- b
    
    return res

func `---`(a: var seq[byte], b: var seq[byte]): seq[byte] =
  # redefine +++ because its needed to peform --- probaly a bad act
  defineBinAdd()
  
  var res: seq[byte] = @[]
  # negative
  if a.len != 0 and b.len != 0 and a[0] == 45 and b[0] == 45:
    a.del(0)
    b.del(0)
    res = @[byte(45)] & a --- b
  # a is negative
  elif a.len != 0 and a[0] == 45:
    a.del(0)
    if a <> b == -1: 
      res = @[byte(45)] & a +++ b
    elif a <> b == 0:
      res = @[byte(0)]
    else:
      res = @[byte(45)] & a +++ b
  
  elif b.len != 0 and b[0] == 45:
    a.del(0)
    if a <> b == -1: 
      res = @[byte(45)] & a +++ b
    elif a <> b == 0:
      res = @[byte(0)]
    else:
      res = @[byte(45)] & a +++ b
  return res

defineBinAdd()

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
          vm.reg.add(@[byte(0)])
        vm.reg[reg0] = @[imm]
        vm.results = success  
        vm.results_eval = $imm
      of OP_ADD:
        var reg0 = bytecode[vm.ip] 
        var reg1 = bytecode[vm.ip + 1] 
        var reg2 = bytecode[vm.ip + 2]
        
        if vm.reg.len - 1 < int(reg0): 
          vm.reg.add(@[byte(0)])     
        vm.reg[reg0] = cast[seq[byte]](cast[int](vm.reg[reg1]) + cast[int](vm.reg[reg2]))
        vm.ip += 3 
        vm.results = success 
        vm.results_eval = $vm.reg[reg0]
      else:
        discard
  
  return vm
