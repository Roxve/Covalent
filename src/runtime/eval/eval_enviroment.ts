import { Enviroment, createEnv } from "../enviroment.ts";
import { Program, Stmt, UseStmt, VarCreation, FuncCreation } from "../../frontend/AST/stmts.ts";
import { eval_var_creation, eval_func_creation, eval_use_stmt } from "./stmt.ts";

export function eval_env(prog: Program) : Enviroment { 
  let env = createEnv();
  for(let stmt of prog.body) {
    evaluate_env(stmt, env);
  }
  return env;
}

export function evaluate_env(node: Stmt, env: Enviroment) {
  switch(node.type) {
    case "VarCreation": 
      return eval_var_creation(node as VarCreation, env);
    case "FuncCreation": 
      return eval_func_creation(node as FuncCreation, env);
    case "UseStmt": 
      return eval_use_stmt(node as UseStmt, env);
    default: 
      return;
  }
}
