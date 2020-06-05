use grass::{SassResult, StyleSheet};
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn compile_from_string(code: String) -> String {
    let sass = StyleSheet::new(code);
    sass.unwrap()
}

#[wasm_bindgen]
pub fn compile_from_file(file: String) -> String {
    let sass = StyleSheet::from_path(&file);
    sass.unwrap()
}
