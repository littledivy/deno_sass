import init, {
  source,
  compile_from_string,
  compile_from_file
} from "./wasm.js";

await init(source);

export function compile(code: string): string {
  return compile_from_string(code);
}

export function compileFile(file: string): string {
  return compile_from_file(file);
}
