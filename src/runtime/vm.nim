import ../compile/codegen_def
import print
import vm_def
import ../etc/utils
import strutils

# template to quickly add binary operations
template BIN_OP(tasks: untyped): untyped =
  var reg0_addr {.inject.} = int(bytecode[vm.ip])
  var reg1_addr {.inject.} = bytecode[vm.ip + 1] 
  vm.checkRegs(reg1_addr)
  var left {.inject.}:const_type = vm.reg[reg0_addr].vtype
  var right {.inject.}: const_type = vm.reg[reg1_addr].vtype
  var reg0 {.inject.}: ptr REG = addr vm.reg[reg0_addr]
  var reg1 {.inject.}: ptr REG = addr vm.reg[reg1_addr]
  tasks
  vm.changeCond(reg0_addr) 
  vm.ip += 2
      
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
            of TAG_STR:
              var count = int(bytecode[vm.ip..vm.ip + 1].makeInt())
              var bytes = bytecode[(vm.ip)..(vm.ip + count + 1)]
              vm.ip += 2 + count
              var str_val = consant(ctype: cstr, bytes: bytes)
              vm.consants.add(str_val)
            else:
              echo "ERROR while loading consts unknown type " & $tag & " please report this!"
              vm.results = UNKNOWN_OP
              vm.results_eval = "INVAILD TAG " & $tag
              return vm

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
        BIN_OP:
          case left:
            of cint:
              reg0.bytes = (makeInt(reg0.bytes) + makeInt(reg1.bytes)).to4Bytes()  
            of cstr: 
              var right_bytes = reg1.bytes
              if right == const_type.cint:
                 print reg1.bytes
                 right_bytes = ($makeInt(reg1.bytes)).StrToBytes 
              reg0.bytes = reg0.bytes & right_bytes
      of OP_SUB:
        BIN_OP:
          case left:
            of cint:
              reg0.bytes = (makeInt(reg0.bytes) - makeInt(reg1.bytes)).to4Bytes()          
            of cstr:
              reg0.bytes = (BytesToStr(reg0.bytes).replace(BytesToStr(reg1.bytes), "")).StrToBytes
      of OP_MUL:
        BIN_OP:
          case left:
            of cint:
              reg0.bytes = (makeInt(reg0.bytes) * makeInt(reg1.bytes)).to4Bytes
            else:
              discard
      of OP_DIV:
        BIN_OP:
          case left:
            of cint:
              reg0.bytes = uint32(int(makeInt(reg0.bytes)) / int(makeInt(reg1.bytes))).to4Bytes    
            else:
              discard 
      else: 
        echo "ERROR while executing: invaild insturaction please report this! " & $op
        vm.results = UNKNOWN_OP
        vm.results_eval = "INVAILD " & $op 
        return vm
  
  return vm
