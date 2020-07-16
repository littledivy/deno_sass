use deno_core::plugin_api::Buf;
use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::ZeroCopyBuf;

use serde::Deserialize;
use serde::Serialize;

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    interface.register_op("compile_str", op_compile);
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
    result: String,
}

fn op_compile(_interface: &mut dyn Interface, data: &[u8], _zero_copy: &mut [ZeroCopyBuf]) -> Op {
    let params: CompileArguments = serde_json::from_slice(data).unwrap();
    let opt = sass_rs::Options {
        output_style: str_to_style(&params.output_style),
        precision: params.precision,
        include_paths: params.include_paths,
        indented_syntax: params.indented_syntax,
    };
    let response = CompileResponse {
        result: sass_rs::compile_string(&params.content, opt).unwrap(),
    };
    let result_box: Buf = serde_json::to_vec(&response).unwrap().into_boxed_slice();
    Op::Sync(result_box)
}

fn str_to_style(style: &str) -> sass_rs::OutputStyle {
    match style {
        "nested" => sass_rs::OutputStyle::Nested,
        "expanded" => sass_rs::OutputStyle::Expanded,
        "compact" => sass_rs::OutputStyle::Compact,
        "compressed" => sass_rs::OutputStyle::Compressed,
        _ => sass_rs::OutputStyle::Nested,
    }
}
