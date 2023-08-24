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
    value: string;
}
export interface Bool extends Expr {
    type: "Bool";
    value: boolean;
}
export interface Null extends Expr {
    type: "Null";
    value: null;
}
