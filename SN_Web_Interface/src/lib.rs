use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert(myvar: String) -> Vec<u8> {
    let test = myvar.as_bytes().to_vec();
    return test;
}