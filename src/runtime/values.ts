export type ValueType = 
| "null"
| "str"
| "num"
| "bool";

export interface RuntimeVal {
  type: ValueType;
  value: any;
}

export interface NumVal extends RuntimeVal {
  type: "num";
  value: number;
}
export interface StrVal extends RuntimeVal {
  type: "str";
  value: string;
}
export interface BoolVal extends RuntimeVal { 
  type: "bool";
  value: boolean;
}
export interface NullVal extends RuntimeVal {
  type: "null";
  value: null;
}

export function MK_NULL() : NullVal {
  return { type: "null", value: null } as NullVal;
}
export function MK_NUM(num: number = 0) : NumVal {
  return { type: "num", value: num } as NumVal;
}
export function MK_STR(str: string = "") : StrVal {
  return { type: "str", value: str } as StrVal;
}
export function MK_BOOL(bool: boolean = false) : BoolVal {
  return { type: "bool", value: bool } as BoolVal;
}
