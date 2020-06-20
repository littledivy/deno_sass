import { compile } from "../mod.ts";

console.log(
  compile(`
    $bg: #eee;
    body {
      color: $bg;
    }
  `, {
    output_style: "compressed",
    precision: 5,
    indented_syntax: false,
    include_paths: []
  }).result,
);
