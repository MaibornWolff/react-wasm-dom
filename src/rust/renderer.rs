use wasm_bindgen::prelude::*;

use super::jsx::Jsx;

#[wasm_bindgen]
pub fn render(jsx: &Jsx) {
    web_sys::console::log_1(jsx);
}