import { Expr } from "./stmts.ts"


export interface Id extends Expr {
    type: "Id";
    symbol: string;
}

export interface Num extends Expr {
    type: "Num";
    value: number;
}

export interface Str extends Expr {
    type: "Str";
    value: number;
}
export interface Null extends Expr {
    type: "Null";
    value: null;
}
