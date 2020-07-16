import { compile_file } from "../mod.ts";

console.log(
    compile_file("examples/_imported.scss", {
      output_style: "compressed",
      precision: 5,
      indented_syntax: false,
      include_paths: [],
    }).result
  );