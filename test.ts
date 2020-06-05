import { assertEquals } from "https://deno.land/std@0.54.0/testing/asserts.ts";
import { compile } from "./mod.ts";

Deno.test({
  name: "compile",
  fn: () => {
    assertEquals(compile("a { b { color: red; } }"), "a b {\n  color: red;\n}\n");
  },
});
