import init, {
  source,
  compile_from_string
} from "./wasm.js";

await init(source);

export function compile(code: string): string {
  return compile_from_string(code);
}
