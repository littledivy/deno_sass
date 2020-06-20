import {
  compile as rs_compile,
} from "./plugin/index.ts";

import { IOptions } from "./types/options.ts";

export function compile(str: string, opts?: IOptions) {
  return rs_compile(str, opts);
}
