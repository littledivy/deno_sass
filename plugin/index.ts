import { prepare } from "../deps.ts";
import { IOptions } from "../types/options.ts";
import { defaultOptions } from "../types/default.ts";

const filenameBase = "deno_sass";

const PLUGIN_URL_BASE =
  "https://github.com/divy-work/deno_sass/releases/latest/download";

const isDev = Deno.env.get("DEV");

if (isDev) {
  let filenameSuffix = ".so";
  let filenamePrefix = "lib";

  if (Deno.build.os === "windows") {
    filenameSuffix = ".dll";
    filenamePrefix = "";
  }
  if (Deno.build.os === "darwin") {
    filenameSuffix = ".dylib";
  }

  const filename =
    `./target/debug/${filenamePrefix}${filenameBase}${filenameSuffix}`;

  // This will be checked against open resources after Plugin.close()
  // in runTestClose() below.
  const resourcesPre = Deno.resources();

  const rid = Deno.openPlugin(filename);
} else {
  const pluginId = await prepare({
    name: "deno_sass",
    printLog: true,
    checkCache: Boolean(Deno.env.get("CACHE")) || true,
    urls: {
      darwin: `${PLUGIN_URL_BASE}/libdeno_sass.dylib`,
      windows: `${PLUGIN_URL_BASE}/deno_sass.dll`,
      linux: `${PLUGIN_URL_BASE}/libdeno_sass.so`,
    },
  });
}

// @ts-ignore
const core = Deno.core as {
  ops: () => { [key: string]: number };
  setAsyncHandler(rid: number, handler: (response: Uint8Array) => void): void;
  dispatch(
    rid: number,
    msg: any,
    buf?: ArrayBufferView,
  ): Uint8Array | undefined;
};

const {
  compile_str,
  compile_file
} = core.ops();

const textDecoder = new TextDecoder();
const textEncoder = new TextEncoder();

export function compile(code: string, opts?: IOptions) {
  let viewOptions: IOptions = Object.assign({}, defaultOptions, opts);
  let view = JSON.stringify({
    content: code,
    ...viewOptions,
  });
  const response = core.dispatch(compile_str, textEncoder.encode(view));
  let res = JSON.parse(textDecoder.decode(response));
  if (res.error) throw new Error(res.error);
  return res;
}

export function compile_from_file(file: string, opts?: IOptions) {
  let viewOptions: IOptions = Object.assign({}, defaultOptions, opts);
  let view = JSON.stringify({
    content: file,
    ...viewOptions,
  });
  const response = core.dispatch(compile_file, textEncoder.encode(view));
  let res = JSON.parse(textDecoder.decode(response));
  if (res.error) throw new Error(res.error);
  return res;
}