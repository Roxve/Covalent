import {Ionizer} from "./frontend/ionizer.ts";
import { Parser } from "./frontend/parser.ts";

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
  while(true) {
    console.log("Atomic");
    const atoms: any = prompt("=>");
    var ionizer = new Ionizer(atoms);
    var ionized = ionizer.ionize();
    console.log(ionized);
  }
}
export function RunTest(atoms: string) {
  var ionizer = new Ionizer(atoms);
  var ionized = ionizer.ionize();
  console.log(ionized);

  let parser: Parser = new Parser(ionized);
  let ev = parser.productAST();
  console.log(ev);
}
// Learn more at https://deno.land/manual/examples/module_metadata#concepts
if (import.meta.main) {
  main(Deno.args);
}
