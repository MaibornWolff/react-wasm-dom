use super::jsx::Jsx;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Component;

    #[wasm_bindgen(method, getter)]
    pub fn props(this: &Component) -> js_sys::Object;

    #[wasm_bindgen(method)]
    pub fn render(this: &Component) -> Jsx;
}
