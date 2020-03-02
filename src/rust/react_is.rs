use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type ReactIs;

    #[wasm_bindgen(method, js_name = isFragment)]
    pub fn is_fragment(react_is: &ReactIs, obj: &JsValue) -> bool;

    #[wasm_bindgen(method, js_name = isElement)]
    pub fn is_element(react_is: &ReactIs, obj: &JsValue) -> bool;
}
