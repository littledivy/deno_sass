extern crate sass_rs;

use deno_core::plugin_api::Buf;
use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::ZeroCopyBuf;

use futures::future::FutureExt;

use serde::Deserialize;
use serde::Serialize;

use std::io::prelude::*;
use std::fs::File;

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    interface.register_op("compile_str", op_compile);
}

#[derive(Deserialize)]
struct CompileArguments {
    content: &str,
    options: sass_rs::Options
}

fn op_compile(
    _interface: &mut dyn Interface,
    data: &[u8],
    _zero_copy: Option<ZeroCopyBuf>,
) -> Op {
    let params: CompileArguments = serde_json::from_slice(data).unwrap();
    sass_rs::compile_string(params.content).unwrap();

    let result = b"true";
    let result_box: Buf = Box::new(*result);
    Op::Sync(result_box)
}
