import { Enviroment } from "./enviroment.ts";

export type ValueType = 
| "null"
| "str"
| "num"
| "bool"
| "obj"
| "functionCall"
| "native-func";

export type ColorType = 
  | "red" 
  | "green" 
  | "white"
  | "yellow";
export interface RuntimeVal {
  type: ValueType;
  value: any;
  color: ColorType;
}

export interface NumVal extends RuntimeVal {
  type: "num";
  value: number;
}


export interface StrVal extends RuntimeVal {
  type: "str";
  value: string;
}


export interface ObjVal extends RuntimeVal {
  type: "obj";
  value: Map<string, RuntimeVal>;
}

export interface BoolVal extends RuntimeVal { 
  type: "bool";
  value: boolean;
}

export interface NullVal extends RuntimeVal {
  type: "null";
  value: null;
}

export type FunctionCall = (
  
  args: RuntimeVal[],
  env: Enviroment
  
) => RuntimeVal;


export interface NativeFnVal extends RuntimeVal{
  type: "native-func";
  call: FunctionCall;
  value: undefined;
}


export function MK_NULL() : NullVal {
  return { type: "null", value: null, color: "red" } as NullVal;
}
export function MK_NUM(num: number = 0) : NumVal {
  return { type: "num", value: num, color: "yellow" } as NumVal;
}
export function MK_STR(str: string = "") : StrVal {
  return { type: "str", value: str, color: "green" } as StrVal;
}
export function MK_BOOL(bool: boolean = false) : BoolVal {
  return { type: "bool", value: bool, color: bool ? "green" : "red" } as BoolVal;
}

export function MK_NATIVE_FUNC(call: FunctionCall) : NativeFnVal {
  return {
    type: "native-func",
    call
  } as NativeFnVal;
}
