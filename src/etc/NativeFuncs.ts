import { MK_NULL, MK_TYPE, RuntimeVal } from "../runtime/values.ts";
import { Enviroment } from "../runtime/enviroment.ts";
import * as Color from "https://deno.land/std@0.200.0/fmt/colors.ts";
import { writeAllSync } from "https://deno.land/std@0.200.0/streams/write_all.ts";

export namespace native {
  //prints the right color
  function print(args: RuntimeVal[]) {
    for (let arg of args) {
      switch (arg.color) {
        case "red":
          writeAllSync(
            Deno.stdout,
            new TextEncoder().encode(Color.red(String(arg.value))),
          );
          break;
        case "green":
          writeAllSync(
            Deno.stdout,
            new TextEncoder().encode(Color.green(String(arg.value))),
          );
          break;
        case "white":
          writeAllSync(
            Deno.stdout,
            new TextEncoder().encode(Color.white(String(arg.value))),
          );
          break;
        case "yellow":
          writeAllSync(
            Deno.stdout,
            new TextEncoder().encode(Color.yellow(String(arg.value))),
          );
          break;
        default:
          writeAllSync(Deno.stdout, new TextEncoder().encode(String(arg.value)));
          break;
      }
    }
  }

  export function write(args: RuntimeVal[], env: Enviroment) {
    print(args);
    //logs a new line
    console.log();

    return MK_NULL();
  }
  export function promptFunc(args: RuntimeVal[], env: Enviroment) {
    print(args);
    let results = prompt("");

    return MK_TYPE(results);
  }
}
