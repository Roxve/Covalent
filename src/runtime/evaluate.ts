import { Stmt, Expr, Program } from "../frontend/AST/stmts.ts";
import { BinaryExpr } from "../frontend/AST/exprs.ts";
import { Id, Num, Null, Str, Bool } from "../frontend/AST/values.ts";
import { Enviroment } from "./enviroment.ts";
import * as VT from "./values.ts";

export function error(msg: string, code: string, stmt: Stmt) : void {
  console.log(`%cRuntime Error:${msg}\nat => line:${stmt.line},colmun:${stmt.colmun},error code:${code}`, 'color: crimson; background-color: gold');
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
    default:
     error("unknown error please report this ", `AT_UNKNOWN_30:${node.type}`, node);
     return VT.MK_NULL();
  }
}
