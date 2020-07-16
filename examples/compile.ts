import { compile } from "../mod.ts";

console.log(
  compile(
    `
    @import "examples/imported";
    $bg: #eee;
    body {
      color: $bg;
    }
  `,
    {
      output_style: "compressed",
      precision: 5,
      indented_syntax: false,
      include_paths: [],
    },
  ).result,
);
