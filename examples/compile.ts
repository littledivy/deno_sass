import { compile } from "../mod.ts";

console.log(compile(`
$bg: #eee;

body {
  color: $bg;
}
`).result)
