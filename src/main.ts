import {Ionizer} from "./frontend/ionizer.ts";
import { Parser } from "./frontend/parser.ts";
import { Enviroment } from "./runtime/enviroment.ts";
import { evaluate } from "./runtime/evaluate.ts";
import { isError, setTest } from "./etc.ts";

function main(args: string[]) {
  if(args === undefined || args === null || args.length <= 0) {
    Repl();
  }

  switch(args[0]) {
    case "run?":
      if(args.length < 2) {
        console.log("file to run excepted.");
      }
      const atoms = Deno.readTextFileSync(args[1]).toString();
      console.log(atoms);
      RunTest(atoms);
      break;
  }
}

function Repl() {
  console.log();
  const env: Enviroment = new Enviroment(null);
  while(true) {
    console.log("%cAtomic", 'color: #c22147');
    const atoms: any = prompt("=>");
    if(atoms == ".exit") {
      Deno.exit(0);
    }
    const ionizer = new Ionizer(atoms);
    const ionized = ionizer.ionize();
    
    const parser: Parser = new Parser(ionized);
    const parsed = parser.productAST();


    const run = evaluate(parsed, env);

    console.log(`%c${run.value}`, `color: ${run.color}`);
  }
}
export function RunTest(atoms: string) {
  setTest();
  const env = new Enviroment(null);
  const ionizer = new Ionizer(atoms);
  const ionized = ionizer.ionize();
  console.log(ionized);

  const parser: Parser = new Parser(ionized);
  const parsed = parser.productAST();
  console.log(parsed);
  if(isError) {
    Deno.exit(1);
  }

  const run = evaluate(parsed, env);
  console.log(run);
}
export function Run(atoms: string) {
  const env = new Enviroment(null);

  const ionizer = new Ionizer(atoms);
  const ionized = ionizer.ionize();

  const parser = new Parser(ionized);
  const parsed = parser.productAST();

  if(isError) {
    Deno.exit(1);
  }
  const run = evaluate(parsed, env)
}
// Learn more at https://deno.land/manual/examples/module_metadata#concepts
if (import.meta.main) {
  main(Deno.args);
}
