import {
  compile as rs_compile,
  compile_from_file as rs_compile_file
} from "./plugin/index.ts";

import { IOptions } from "./types/options.ts";

export function compile(str: string, opts?: IOptions) {
  return rs_compile(str, opts);
}

export function compile_file(file: string, opts?: IOptions) {
  return rs_compile_file(file, opts);
}