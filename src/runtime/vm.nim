import ../compile/codegen_def
import print
import vm_def
import ../etc/utils
import strutils
import ../etc/enviroments
import tables
# template to quickly add binary operations
template BIN_OP(tasks: untyped): untyped =
  var reg0_addr {.inject.} = system.int(bytecode[vm.ip])
  var reg1_addr {.inject.} = bytecode[vm.ip + 1] 
  vm.checkRegs(reg1_addr)
  var left {.inject.}:ValueType = vm.reg[reg0_addr].kind
  var right {.inject.}: ValueType = vm.reg[reg1_addr].kind
  var reg0 {.inject.}: ptr REG = addr vm.reg[reg0_addr]
  var reg1 {.inject.}: ptr REG = addr vm.reg[reg1_addr]
  tasks
  vm.changeCond(reg0_addr) 
  vm.ip += 2
  
proc interpret*(bytecode: seq[byte]): VM =
  var vm = VM()
  vm.ip = 0
  dprint: bytecode

  var env = Enviroment(varibles: Table[uint16, RuntimeValue]()) 
  while vm.ip < bytecode.len:
    var op = OP(bytecode[vm.ip])    
    dprint: op
    vm.ip += 1
    case op: 
      of OP_CONSTS:
        var consts_count = makeInt(bytecode[vm.ip..vm.ip + 1])
        vm.ip += 2
        dprint: consts_count
    
        for i in 0 .. consts_count:
          dprint: i
          var tag = OP(bytecode[vm.ip])
          vm.ip += 1
          case tag:
            of TAG_INT:
              var bytes = bytecode[vm.ip .. vm.ip + 3]
              var int_val = RuntimeValue(kind: int, bytes: bytes)

        
              dprint: bytes
              vm.consts.add(int_val)
              vm.ip += 4
            of TAG_STR:
              var count = bytecode[vm.ip..vm.ip + 1].makeInt()
              var bytes = bytecode[(vm.ip)..(vm.ip + count + 1)]
              vm.ip += 2 + count

              
              var str_val = RuntimeValue(kind: str, bytes: bytes)
              vm.consts.add(str_val)
            of TAG_FLOAT:
              var bytes = bytecode[vm.ip .. vm.ip + 3]            
              dprint: makeFloat(bytes)
              
              vm.ip += 4

              var float_val = RuntimeValue(kind: float, bytes: bytes)
              vm.consts.add(float_val)
              
            else:
              echo "ERROR while loading consts unknown type " & $tag & " please report this!"
              vm.results = UNKNOWN_OP
              vm.results_eval = "INVAILD TAG " & $tag
              return vm
      of OP_STRNAME:
        var count = uint16(makeInt(bytecode[vm.ip..vm.ip + 1]))

        var regip = bytecode[vm.ip]
        vm.checkRegs(regip)
        var reg = addr vm.reg[regip]

        vm.ip += 3

        env.setVar(count, RuntimeValue(kind: reg.kind, bytes: reg.bytes))
      of OP_LOADNAME:
        var regip = bytecode[vm.ip]
        vm.checkRegs(regip)
        var reg = addr vm.reg[regip]
        var index = uint16(makeInt(bytecode[vm.ip + 1..vm.ip + 2]))
        vm.ip += 3
        
        var val = env.getVarVal(index)
        reg.bytes = val.bytes
        reg.kind = val.kind
    
      of OP_LOAD_CONST:
        var reg0 = bytecode[vm.ip]
        var imm = makeInt(bytecode[vm.ip + 1..vm.ip + 2])
        vm.ip += 3
    
        vm.checkRegs(reg0)
        var consts = vm.consts[imm - 1] 
      
        vm.reg[reg0] = REG(kind: consts.kind, bytes: consts.bytes)
        vm.changeCond(reg0)
      of OP_LOAD:
        var reg0 = bytecode[vm.ip]
      
        var imm = bytecode[vm.ip + 1] 
      
        vm.ip += 2
        vm.checkRegs(reg0)
        
        vm.reg[reg0].bytes = @[imm]
        vm.changeCond(reg0)
        
        
      of OP_ADD:
        BIN_OP:
          case left:
            of int:
              var num1 = makeInt(reg0.bytes)           
              var num2 = makeInt(reg1.bytes)            
              reg0.bytes = to4Bytes(num1 + num2)  
            of float:
              var num1 = makeFloat(reg0.bytes)
              var num2 = makeFloat(reg1.bytes)
              reg0.bytes = to4Bytes(num1 + num2)
            of str: 
              var right_bytes = reg1.bytes
              if right == ValueType.int:
                 right_bytes = ($makeInt(reg1.bytes)).StrToBytes
              elif right == ValueType.float:
                 right_bytes = ($makeFloat(reg1.bytes)).StrToBytes 
              reg0.bytes = reg0.bytes & right_bytes
            else:
              discard
      of OP_SUB:
        BIN_OP:
          case left:
            of int:
              var num1 = makeInt(reg0.bytes)
              var num2 = makeInt(reg1.bytes) 
              reg0.bytes = to4Bytes(num1 - num2)   
            of float:
              var num1 = makeFloat(reg0.bytes)
              var num2 = makeFloat(reg1.bytes)
              reg0.bytes = to4Bytes(num1 - num2)      
            of str:
              var str1 = BytesToStr(reg0.bytes)
              var str2 = BytesToStr(reg1.bytes)              
              reg0.bytes = StrToBytes(str1.replace(str2, ""))
            else:
              discard
      of OP_MUL: 
        BIN_OP:
          case left:
            of int:
              var num1 = makeInt(reg0.bytes)    
              var num2 = makeInt(reg1.bytes)     
              reg0.bytes = to4Bytes(num1 * num2)
            of float:
              var num1 = makeFloat(reg0.bytes)
              var num2 = makeFloat(reg1.bytes)
              reg0.bytes = to4Bytes(num1 * num2)    
            else:
              discard
      of OP_DIV:
        BIN_OP:
          case left:
            of int:
              var num1 = makeInt(reg0.bytes) 
              var num2 = makeInt(reg1.bytes)
              reg0.bytes = to4Bytes(float32(num1 / num2))
              reg0.kind = float
            of float:
              var num1 = makeFloat(reg0.bytes)
              var num2 = makeFloat(reg1.bytes)
              reg0.bytes = to4Bytes(float32(num1 / num2))
            else:
              discard 
      else: 
        echo "ERROR while executing: invaild insturaction please report this! " & $op
        vm.results = UNKNOWN_OP
        vm.results_eval = "INVAILD " & $op 
        return vm
  
  return vm
