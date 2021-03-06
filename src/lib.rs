use deno_core::plugin_api::Buf;
use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::ZeroCopyBuf;

use serde::Deserialize;
use serde::Serialize;

mod utils;

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    interface.register_op("compile_str", op_compile);
    interface.register_op("compile_file", op_compile_file);
}

#[derive(Deserialize)]
struct CompileArguments {
    content: String,
    output_style: String,
    precision: usize,
    indented_syntax: bool,
    include_paths: Vec<String>,
}

#[derive(Serialize)]
struct CompileResponse {
    result: Option<String>,
    error: Option<String>,
}

fn op_compile(_interface: &mut dyn Interface, data: &[u8], _zero_copy: &mut [ZeroCopyBuf]) -> Op {
    let params: CompileArguments = serde_json::from_slice(data).unwrap();
    let opt = sass_rs::Options {
        output_style: utils::str_to_style(&params.output_style),
        precision: params.precision,
        include_paths: params.include_paths,
        indented_syntax: params.indented_syntax,
    };
    let response: CompileResponse;
    match sass_rs::compile_string(&params.content, opt) {
        Ok(r) => {
            response = CompileResponse {
                result: Some(r),
                error: None,
            };
        }
        Err(e) => {
            response = CompileResponse {
                result: None,
                error: Some(e.to_string()),
            };
        }
    }
    let result_box: Buf = serde_json::to_vec(&response).unwrap().into_boxed_slice();
    Op::Sync(result_box)
}


fn op_compile_file(_interface: &mut dyn Interface, data: &[u8], _zero_copy: &mut [ZeroCopyBuf]) -> Op {
    let params: CompileArguments = serde_json::from_slice(data).unwrap();
    let opt = sass_rs::Options {
        output_style: utils::str_to_style(&params.output_style),
        precision: params.precision,
        include_paths: params.include_paths,
        indented_syntax: params.indented_syntax,
    };
    let response: CompileResponse;
    match sass_rs::compile_file(&params.content, opt) {
        Ok(r) => {
            response = CompileResponse {
                result: Some(r),
                error: None,
            };
        }
        Err(e) => {
            response = CompileResponse {
                result: None,
                error: Some(e.to_string()),
            };
        }
    }
    let result_box: Buf = serde_json::to_vec(&response).unwrap().into_boxed_slice();
    Op::Sync(result_box)
}
