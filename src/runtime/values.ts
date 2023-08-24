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
