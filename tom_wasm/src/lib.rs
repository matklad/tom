#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate wasm_bindgen;
extern crate tom;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(text: &str) -> String {
    tom::TomlDoc::new(text).debug()
}
