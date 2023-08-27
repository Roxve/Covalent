//import { assertEquals } from "https://deno.land/std@0.199.0/assert/mod.ts";
import { RunTest } from "./main.ts";

Deno.test(function AtomsTest() {
  RunTest(Deno.readTextFileSync("../TestProj/main.atoms"));
});
