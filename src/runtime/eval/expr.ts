import * as VT from "../values.ts";
import * as AST from "../../frontend/AST/exprs.ts";
import * as ASTV from "../../frontend/AST/values.ts";
import { evaluate } from "../evaluate.ts";
import { Expr, Stmt } from "../../frontend/AST/stmts.ts";
import { Enviroment } from "../enviroment.ts";
import { RuntimeVal, MK_NULL } from "../values.ts";
import { error } from "../evaluate.ts";



export function eval_assign_expr(expr: AST.AssignExpr, env: Enviroment) {
  if(expr.assigne.type != "Id") {
    error("excepted id(var name) to assigne to in assignment expr", "AT3004", expr);
    return MK_NULL();
  }
  let name = (expr.assigne as ASTV.Id).symbol;
  
  return env.setVar(name, evaluate(expr.value, env), expr);
}


export function eval_object(expr: ASTV.Object, env: Enviroment) : RuntimeVal {
  let properties: Map<string, RuntimeVal> = new Map();

  for(let property of expr.properties) {
    let value: RuntimeVal;
    if(property.value === null) {
      value = env.findVar(property.key, property);
    }
    else {
      value = evaluate(property.value, env);
    }
    properties.set(property.key, value);
  }
  
  return {
    type: "obj",
    value: properties,
    color: "yellow"
  } as VT.ObjVal;  
}



export function eval_binary_expr(expr: AST.BinaryExpr, env: Enviroment) : VT.RuntimeVal {
  const lhs: VT.RuntimeVal = evaluate(expr.left, env); const rhs: VT.RuntimeVal = evaluate(expr.right, env);
  switch(expr.ooperator) {
    case "+":
      return eval_plus_binary_expr(lhs, rhs, expr.left);
    case "-":
      return eval_minus_binary_expr(lhs,rhs, expr);
    case "*":
      return eval_multy_binary_expr(lhs, rhs, expr);
    case "/":
      return eval_divide_binary_expr(lhs, rhs, expr);
    default:
      return MK_NULL();
  }
}

export function eval_plus_binary_expr(lhs: RuntimeVal, rhs: RuntimeVal, expr: Expr) : VT.RuntimeVal {
  if(lhs.type === "str" || rhs.type === "str") {
    return VT.MK_STR(lhs.value + rhs.value);
  }
  else if(rhs.type === "num" && lhs.type === "num") {
    return VT.MK_NUM(lhs.value + rhs.value);
  }
  else {
    error(`cannot beform ooperation plus on right hand of type:${rhs.type} && left hand of type:${lhs.type}`, "AT3003", expr);
    return MK_NULL();
  }
}

export function eval_minus_binary_expr(lhs: RuntimeVal, rhs: RuntimeVal, expr: Expr) {
  if(lhs.type === "num" && rhs.type === "num") {
    return VT.MK_NUM(lhs.value - rhs.value);
  }
  else if(lhs.type === "str") {
    return VT.MK_STR((lhs as VT.StrVal).value.replace(rhs.value, ""));
  }
  else {
    error(`cannot beform ooperation minus on left hand of type:${lhs.type}, right hand of type:${rhs.type}`, "AT3003", expr);
    return MK_NULL();
  }
}

export function eval_multy_binary_expr(lhs: RuntimeVal, rhs: RuntimeVal,expr: Expr) : RuntimeVal {
  if(lhs.type === "num" && rhs.type === "num") {
    return VT.MK_NUM(lhs.value * rhs.value);
  }
  else {
    error(`cannot beform ooperation multiply on left hand of type:${lhs.type}, right hand of type:${rhs.type}`, "AT3003", expr);
    return MK_NULL();
  }
}
export function eval_divide_binary_expr(lhs: RuntimeVal, rhs: RuntimeVal, expr: Expr) {
  if(lhs.type === "num" && rhs.type === "num") {
    if(rhs.value === 0) {
      // TODO add a warning here
      return VT.MK_NUM(0);
    }
    return VT.MK_NUM(rhs.value / lhs.value);
  }
  else {
    error(`cannot beform ooperation divide on left hand of type:${lhs.type} && right hand of type:${rhs.type}`, "AT3003", expr);
    return MK_NULL();
  }
}

export function eval_member_expr(expr: AST.MemberExpr, env: Enviroment) : RuntimeVal {
   let obj = evaluate(expr.obj, env);

   if(obj.type != "obj") {
     error(`excepted an obj in MemberExpr`, "AT3007", expr);
     return MK_NULL();
   }
   if(expr.isIndexed) {
     if(expr.property.type != "Num") {
       error("excepted index of num in indexed MemberExpr", "AT3007", expr);
       return MK_NULL();
     }
      return env.getObjProperty(obj as VT.ObjVal, "null",expr, (expr.property as ASTV.Num).value)
   }
   else if(expr.property.type == "MemberExpr") {
     return eval_member_expr(expr, env);
   }
   else if(expr.property.type == "Id") {
     return env.getObjProperty(obj as VT.ObjVal, (expr.property as ASTV.Id).symbol, expr);
   }
   return MK_NULL();
}
