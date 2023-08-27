import { RuntimeVal, MK_NULL, FnVal } from "../values.ts";
import { evaluate, error } from "../evaluate.ts";
import { VarCreation, FuncCreation } from "../../frontend/AST/stmts.ts";
import { Enviroment } from "../enviroment.ts";
export function eval_var_creation(stmt: VarCreation, env: Enviroment) : RuntimeVal {
  let value: RuntimeVal = evaluate(stmt.value, env);
  
  return env.declareVar(stmt.name, value, stmt.isLocked, stmt); 
}

export function eval_func_creation(stmt: FuncCreation, env: Enviroment) : RuntimeVal {
  return env.declareVar(stmt.name, {
    type: "func",
    name: stmt.name,
    parameters: stmt.parameters,
    body: stmt.body,
    env,
    value: undefined
  } as FnVal, true, stmt);
}
