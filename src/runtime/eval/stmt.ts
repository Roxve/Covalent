import { RuntimeVal, MK_NULL } from "../values.ts";
import { evaluate, error } from "../evaluate.ts";
import { VarCreation } from "../../frontend/AST/stmts.ts";
import { Enviroment } from "../enviroment.ts";
export function eval_var_creation(stmt: VarCreation, env: Enviroment) : RuntimeVal {
  let value: RuntimeVal = evaluate(stmt.value, env);
  
  return env.declareVar(stmt.name, value, stmt.isLocked, stmt); 
}
