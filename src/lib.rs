extern crate sass_rs;

use deno_core::plugin_api::Buf;
use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::ZeroCopyBuf;

use futures::future::FutureExt;

use serde::Deserialize;
use serde::Serialize;

use std::fs::File;
use std::io::prelude::*;

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    interface.register_op("compile_str", op_compile);
}

#[derive(Deserialize)]
struct CompileArguments {
    content: String,
    // output_style: sass_rs::OutputStyle,
    // precision: usize,
    // indented_syntax: bool,
    // include_paths: Vec<String>,
}

#[derive(Serialize)]
struct CompileResponse {
    result: String,
}

fn op_compile(_interface: &mut dyn Interface, data: &[u8], _zero_copy: &mut [ZeroCopyBuf]) -> Op {
    let params: CompileArguments = serde_json::from_slice(data).unwrap();
    let opt = sass_rs::Options {
        output_style: sass_rs::OutputStyle::Nested,
        precision: 10,
        include_paths: vec!["/".to_string()],
        indented_syntax: true,
    };
    let mut response = CompileResponse {
        result: sass_rs::compile_string(&params.content, sass_rs::Options::default()).unwrap(),
    };
    let result_box: Buf = serde_json::to_vec(&response).unwrap().into_boxed_slice();
    Op::Sync(result_box)
}
