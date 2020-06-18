import {
  compile as rs_compile,
} from "./plugin/index.ts";

export function compile(str: string) {
  return rs_compile(str);
}
