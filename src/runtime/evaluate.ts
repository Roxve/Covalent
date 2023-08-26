import { Stmt, Expr, Program, VarCreation } from "../frontend/AST/stmts.ts";
import { BinaryExpr } from "../frontend/AST/exprs.ts";
import { Id, Num, Null, Str, Bool } from "../frontend/AST/values.ts";
import { Enviroment } from "./enviroment.ts";
import * as VT from "./values.ts";
import * as expr from "./eval/expr.ts";
import * as stmt from "./eval/stmt.ts";
import { createError } from "../etc.ts";

export function error(msg: string, code: string, stmt: Stmt) : void {
  createError(`Runtime Error:${msg}\nat => line:${stmt.line},colmun:${stmt.colmun},error code:${code}`);
}

export function eval_program(prog: Program, env: Enviroment) : VT.RuntimeVal {
  let results: VT.RuntimeVal = VT.MK_NULL();
  prog.body.forEach(function(stmt: Stmt) {
    results = evaluate(stmt, env);
  });
  return results;
}
export function evaluate(node: Stmt, env: Enviroment) : VT.RuntimeVal {
  switch(node.type) {
    case "Id":
      return env.findVar((node as Id).symbol, node as Id);
    case "Program":
      return eval_program(node as Program, env);
    case "Null":
      return VT.MK_NULL();
    case "Bool":
      return VT.MK_BOOL((node as Bool).value);
    case "Str":
      return VT.MK_STR((node as Str).value);
    case "Num":
      return VT.MK_NUM((node as Num).value);
    case "VarCreation":
      return stmt.eval_var_creation(node as VarCreation, env);
    case "BinaryExpr":
      return expr.eval_binary_expr(node as BinaryExpr, env);
    default:
     error("unknown error please report this ", `AT_UNKNOWN_30:${node.type}`, node);
     console.log(node);
     return VT.MK_NULL();
  }
}
