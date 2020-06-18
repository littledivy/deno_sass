extern crate tar;

use deno_core::plugin_api::Buf;
use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::ZeroCopyBuf;

use futures::future::FutureExt;

use serde::Deserialize;
use serde::Serialize;

use std::io::prelude::*;
use std::fs::File;
use tar::{Archive, Builder};

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    interface.register_op("archive", op_archive);
    interface.register_op("read_archive", op_read_archive);
}

#[derive(Deserialize)]
struct ArchiveOptions {
    tarname: String,
    files: Vec<String>
}

fn op_archive(
    _interface: &mut dyn Interface,
    data: &[u8],
    _zero_copy: Option<ZeroCopyBuf>,
) -> Op {
    let params: ArchiveOptions = serde_json::from_slice(data).unwrap();

    let file = File::create(params.tarname).unwrap();
    let mut a = Builder::new(file);
    for i in params.files {
       a.append_path(i).unwrap();
    }
    // a.append_file("file2.txt", &mut File::open("file3.txt").unwrap()).unwrap();
    let result = b"true";
    let result_box: Buf = Box::new(*result);
    Op::Sync(result_box)
}

#[derive(Serialize)]
struct ReadResponse {
    filenames: Vec<String>
}

fn op_read_archive(
    _interface: &mut dyn Interface,
    data: &[u8],
    _zero_copy: Option<ZeroCopyBuf>,
) -> Op {
    let data_str = std::str::from_utf8(&data[..]).unwrap().to_string();
    let file = File::open(data_str).unwrap();
    let mut a = Archive::new(file);
    let mut filenames: Vec<String> = [].to_vec();
    for file in a.entries().unwrap() {
        // Make sure there wasn't an I/O error
        let mut file = file.unwrap();

        // Inspect metadata about the file
        // println!("{:?}", file.header().path().unwrap());
        // println!("{}", file.header().size().unwrap());
        filenames.push(file.header().path().unwrap().to_str().unwrap().to_string());

        // files implement the Read trait
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        // println!("{}", s);
    }
    let response = ReadResponse {
        filenames: filenames
    };
    let result_box: Buf = serde_json::to_vec(&response).unwrap().into_boxed_slice();
    Op::Sync(result_box)
}
