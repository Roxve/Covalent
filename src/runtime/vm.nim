import ../compile/codegen
import ../compile/codegen_def
import print
import vm_def

proc interpret*(bytecode: seq[byte]): VM =
  var vm = VM()
  vm.ip = 0
  print bytecode
  while vm.ip < bytecode.len:
    var op = OP(bytecode[vm.ip])    
    print op
    vm.ip += 1
    case op: 
      of OP_CONSANTS:
        var consants_count = makeInt(bytecode[vm.ip..vm.ip + 1])
        vm.ip += 2
        print consants_count
    
        for i in 0 .. consants_count:
          print i
          var tag = OP(bytecode[vm.ip])
          vm.ip += 1
          case tag:
            of TAG_INT:
              var bytes = bytecode[vm.ip .. vm.ip + 3]
              var int_val = consant(ctype: cint, bytes: bytes)
              print bytes
              vm.consants.add(int_val)
              vm.ip += 4
            else:
              discard


      of OP_LOAD_CONST:
        var reg0 = bytecode[vm.ip]
        var imm = makeInt(bytecode[vm.ip + 1..vm.ip + 2])
        vm.ip += 3
    
        vm.checkRegs(reg0)
        var constant = vm.consants[imm - 1] 
      
        vm.reg[reg0] = REG(vtype: constant.ctype, bytes: constant.bytes)
        vm.changeCond(int(reg0))
      of OP_LOAD:
        var reg0 = bytecode[vm.ip]
        print reg0
        var imm = bytecode[vm.ip + 1] 
        print imm
        vm.ip += 2
        vm.checkRegs(reg0)
        
        vm.reg[reg0].bytes = @[imm]
        vm.changeCond(int(reg0))
        print vm.reg
        
      of OP_ADD:
        var dist = int(bytecode[vm.ip])
        var reg1 = bytecode[vm.ip + 1] 
        vm.checkRegs(reg1)
        case vm.reg[dist].vtype:
          of cint:
            vm.reg[dist].bytes = (makeInt(vm.reg[dist].bytes) + makeInt(vm.reg[reg1].bytes)).to4Bytes()    
        vm.changeCond(dist) 
        
        vm.ip += 2       
      else:
        discard
  
  return vm
