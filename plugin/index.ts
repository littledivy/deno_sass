import { prepare } from "../deps.ts";
import { ArchiveParams } from "../types.ts";

const filenameBase = "deno_tar";

const PLUGIN_URL_BASE =
  "https://github.com/divy-work/deno-tar/releases/latest/download";

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
    name: "autopilot_deno",
    printLog: true,
    checkCache: Boolean(Deno.env.get("CACHE")) || true,
    urls: {
      darwin: `${PLUGIN_URL_BASE}/libautopilot_deno.dylib`,
      windows: `${PLUGIN_URL_BASE}/autopilot_deno.dll`,
      linux: `${PLUGIN_URL_BASE}/libautopilot_deno.so`,
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
  archive,
  read_archive
} = core.ops();

const textDecoder = new TextDecoder();
const textEncoder = new TextEncoder();

export function TarArchive(params: ArchiveParams) {
  const response = core.dispatch(archive, textEncoder.encode(JSON.stringify(params)));
  return JSON.parse(textDecoder.decode(response));
}

export function TarReadArchive(params: any) {
  const response = core.dispatch(read_archive, textEncoder.encode(params));
  return JSON.parse(textDecoder.decode(response));
}
